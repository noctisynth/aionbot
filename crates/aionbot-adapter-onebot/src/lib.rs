pub extern crate aionbot_core;

pub mod ws;

use std::sync::Arc;

use aionbot_core::event::Event;
use aionbot_core::runtime::{Runtime, RuntimeStatus, StateManager};
use anyhow::Result;
use tokio::sync::broadcast::Receiver;
use ws::Onebot;

pub trait Adapter {
    fn reply(&self, message: &str) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Adapter for dyn aionbot_core::event::Event {
    async fn reply(&self, message: &str) -> Result<()> {
        let _ = message;
        // let ws = onebot_v11::connect::ws_reverse::ReverseWsConnect::new(config);
        unimplemented!()
    }
}

pub struct OnebotRuntime {
    onebot: Option<Arc<Onebot>>,
    state: Arc<StateManager>,
    receiver: Option<Receiver<Event>>,
}

impl Default for OnebotRuntime {
    fn default() -> Self {
        Self {
            onebot: None,
            state: Arc::new(StateManager::default()),
            receiver: None,
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
        println!("Preparing Onebot runtime");
        self.onebot = Some(ws::Onebot::new().listen(Default::default()).await?);
        println!("Onebot runtime prepared");
        Ok(())
    }

    fn setup(&mut self, setup: Box<dyn FnOnce(&Self) + Send + Sync>) {
        setup(self);
    }

    async fn finalize(&mut self) -> Result<()> {
        self.receiver = Some(self.onebot.as_ref().cloned().unwrap().subscribe().await);
        Ok(())
    }

    async fn run(&mut self) -> Result<RuntimeStatus> {
        let event = self.receiver.as_mut().unwrap().recv().await?;
        Ok(RuntimeStatus::Event(event))
    }
}
