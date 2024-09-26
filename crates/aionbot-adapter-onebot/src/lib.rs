use std::sync::Arc;

use aionbot_core::runtime::{Runtime, RuntimeStatus, StateManager};
use anyhow::Result;
use onebot_v11::connect::ws_reverse::ReverseWsConnect;

pub extern crate aionbot_core;

pub trait Adapter {
    fn reply(&self, message: &str) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Adapter for aionbot_core::event::Event {
    async fn reply(&self, message: &str) -> Result<()> {
        let _ = message;
        // let ws = onebot_v11::connect::ws_reverse::ReverseWsConnect::new(config);
        unimplemented!()
    }
}

pub struct OnebotRuntime {
    connect: Option<Arc<ReverseWsConnect>>,
    state: Arc<StateManager>,
}

impl Default for OnebotRuntime {
    fn default() -> Self {
        Self {
            connect: None,
            state: Arc::new(StateManager::default()),
        }
    }
}

impl Runtime for OnebotRuntime {
    fn set_manager(mut self, manager: Arc<StateManager>) -> Self {
        self.state = manager;
        self
    }

    fn manager(&self) -> &StateManager {
        &self.state
    }

    async fn prepare(&mut self) -> Result<()> {
        self.connect = Some(ReverseWsConnect::new(Default::default()).await?);
        Ok(())
    }

    fn setup(&mut self, setup: Box<dyn FnOnce(&Self) + Send + Sync>) {
        setup(self);
    }

    async fn finalize(&mut self) -> Result<()> {
        Ok(())
    }

    async fn run(&self) -> Result<RuntimeStatus> {
        Ok(RuntimeStatus::Exit)
    }
}
