use std::sync::Arc;

use anyhow::Result;
use state::TypeMap;

use crate::{entry::Entry, event::Event, handler::Handler, types::SetupFn};

#[derive(Default)]
pub struct StateManager(pub(crate) TypeMap!(Send + Sync));

impl StateManager {
    pub fn new() -> Self {
        StateManager(<TypeMap![Send + Sync]>::new())
    }

    pub fn set<T: Send + Sync + 'static>(&mut self, state: T) {
        self.0.set::<T>(state);
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> &T {
        self.0.get::<T>()
    }

    pub fn try_get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.0.try_get::<T>()
    }

    pub fn get_owned<T: Clone + Send + Sync + 'static>(&self) -> T {
        self.0.get::<T>().to_owned()
    }
}

pub struct Builder<R: Runtime + Default> {
    handler: Arc<Option<Handler>>,
    runtime: R,
    state: Arc<StateManager>,
    setup: Option<SetupFn<R>>,
}

impl<R> Builder<R>
where
    R: Runtime + Default + Send,
{
    pub fn setup(&mut self, setup: SetupFn<R>) {
        self.setup = Some(setup);
    }

    pub fn invoke_handler(mut self, entries: Vec<Entry>) -> Self {
        self.handler = Arc::new(Some(Handler::new(entries)));
        self
    }

    pub fn manage<T: Send + Sync + 'static>(self, state: T) -> Self {
        self.state.0.set(state);
        self
    }

    async fn prepare(&mut self) -> Result<()> {
        self.runtime.prepare().await?;
        if let Some(setup) = self.setup.take() {
            self.runtime.setup(setup);
        }
        self.runtime.finalize().await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        self.prepare().await?;

        loop {
            match self.runtime.run().await? {
                RuntimeStatus::Exit => break,
                RuntimeStatus::Next => {}
                RuntimeStatus::Restart => {
                    self.runtime.prepare().await?;
                }
                RuntimeStatus::Event(event) => {
                    let handler = self.handler.clone();
                    tokio::spawn(async move {
                        handler
                            .as_ref()
                            .clone()
                            .unwrap()
                            .input(&Arc::new(event))
                            .await
                            .unwrap();
                    });
                }
            }
        }
        Ok(())
    }
}

impl<R> Default for Builder<R>
where
    R: Runtime + Default + Send + 'static,
{
    fn default() -> Self {
        let manager = Arc::new(StateManager::new());
        let runtime = R::default().set_manager(manager.clone());
        Self {
            handler: Arc::new(None),
            runtime,
            state: Arc::clone(&manager),
            setup: None,
        }
    }
}

pub trait Runtime {
    #[must_use]
    fn set_manager(self, manager: Arc<StateManager>) -> Self;

    fn manager(&self) -> &StateManager;

    fn prepare(&mut self) -> impl std::future::Future<Output = Result<()>> + Send {
        async move { Ok(()) }
    }

    fn setup(&mut self, setup: SetupFn<Self>) {
        setup(self)
    }

    fn finalize(&mut self) -> impl std::future::Future<Output = Result<()>> + Send {
        async move { Ok(()) }
    }

    fn run(&mut self) -> impl std::future::Future<Output = Result<RuntimeStatus>> + Send;
}

pub enum RuntimeStatus {
    Next,
    Exit,
    Restart,
    Event(Event),
}
