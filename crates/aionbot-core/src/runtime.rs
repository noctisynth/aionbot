use std::{cell::UnsafeCell, sync::Arc};

use anyhow::Result;
use state::TypeMap;

use crate::{entry::Entry, event::Event, handler::Handler, plugin::AionPlugin, types::SetupFn};

#[derive(Default)]
pub struct StateManager(pub(crate) TypeMap!(Send + Sync));

impl StateManager {
    pub fn new() -> Self {
        StateManager(<TypeMap![Send + Sync]>::new())
    }

    pub fn set<T: Send + Sync + 'static>(&self, state: T) {
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
    handler: UnsafeCell<Handler>,
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

    pub fn invoke_handler<E: IntoIterator<Item = Entry>>(mut self, entries: E) -> Self {
        self.handler.get_mut().extend(entries);
        self
    }

    pub fn plugin(self, plugin: AionPlugin) -> Self {
        self.invoke_handler(plugin.entries().to_vec())
    }

    pub fn manage<T: Send + Sync + 'static>(self, state: T) -> Self {
        self.state.set(state);
        self
    }

    async fn prepare(&mut self) -> Result<()> {
        log::debug!("Preparing for runtime...");
        self.runtime.prepare().await?;
        if let Some(setup) = self.setup.take() {
            log::debug!("Setting up runtime...");
            self.runtime.setup(setup);
        }
        log::debug!("Finalizing runtime...");
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
                    log::info!("Restarting bot runtime...");
                    self.runtime.prepare().await?;
                }
                RuntimeStatus::Event(event) => {
                    let handler = unsafe { self.handler.get().as_mut() }.unwrap();
                    tokio::spawn(async move {
                        if let Err(e) = handler.input(Arc::new(event)).await {
                            log::error!("Error handling event: {}", e);
                        };
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
            handler: UnsafeCell::new(Handler::empty()),
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
    Event(Box<dyn Event>),
}
