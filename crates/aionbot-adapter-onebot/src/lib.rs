use std::sync::Arc;

use aionbot_core::runtime::{Runtime, StateManager};
use anyhow::Result;

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

#[derive(Default)]
pub struct OnebotRuntime {
    state: Arc<StateManager>,
}

impl Runtime for OnebotRuntime {
    fn set_manager(mut self, manager: Arc<StateManager>) -> Self {
        self.state = manager;
        self
    }

    fn manager(&self) -> &StateManager {
        &self.state
    }

    async fn run(&self) -> Result<()> {
        Ok(())
    }
}
