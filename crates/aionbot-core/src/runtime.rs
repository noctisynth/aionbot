use std::sync::Arc;

use anyhow::Result;
use state::TypeMap;

use crate::{entry::Entry, handler::Handler};

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
}

pub struct Builder<R: Runtime + Default> {
    handler: Arc<Option<Handler>>,
    runtime: R,
    state: Arc<StateManager>,
}

impl<R> Builder<R>
where
    R: Runtime + Default + Send + 'static,
{
    pub fn invoke_handler(mut self, entries: Vec<Entry>) -> Self {
        self.handler = Arc::new(Some(Handler::new(entries)));
        self
    }

    pub fn manage<T: Send + Sync + 'static>(self, state: T) -> Self {
        self.state.0.set(state);
        self
    }

    pub async fn run(&mut self) -> Result<()> {
        self.runtime.run().await
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
        }
    }
}

pub trait Runtime {
    #[must_use]
    fn set_manager(self, manager: Arc<StateManager>) -> Self;

    fn manager(&self) -> &StateManager;

    fn setup(&mut self, setup: Box<dyn FnOnce(&Self) + Send + Sync>) {
        setup(self)
    }

    fn run(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}
