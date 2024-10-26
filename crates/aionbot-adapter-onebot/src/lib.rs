pub extern crate aionbot_core;

pub mod event;
pub mod models;
pub mod ws;

use std::{any::Any, sync::Arc};

use aionbot_core::{
    event::Event,
    runtime::{Runtime, RuntimeStatus, StateManager},
};
use anyhow::Result;
use event::OnebotEvent;
use tokio::sync::broadcast::Receiver;
use ws::Onebot;

pub trait Adapter: Any {
    fn reply(&self, message: &str) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Adapter for dyn Event {
    async fn reply(&self, message: &str) -> Result<()> {
        let event = unsafe { (self as *const dyn Event as *mut OnebotEvent).as_mut() }.unwrap();
        let bot = event.bot.clone();

        bot.send(event, message).await;
        Ok(())
    }
}

pub struct OnebotRuntime {
    onebot: Option<Arc<Onebot>>,
    state: Arc<StateManager>,
    receiver: Option<Receiver<Box<OnebotEvent>>>,
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
        log::debug!("Preparing for Onebot runtime...");
        self.onebot = Some(ws::Onebot::new().listen(Default::default()).await?);
        Ok(())
    }

    fn setup(&mut self, setup: Box<dyn FnOnce(&Self) + Send + Sync>) {
        setup(self);
    }

    async fn finalize(&mut self) -> Result<()> {
        log::debug!("Finalizing Onebot runtime..");
        self.receiver = Some(self.onebot.as_ref().cloned().unwrap().subscribe().await);
        Ok(())
    }

    async fn run(&mut self) -> Result<RuntimeStatus> {
        log::debug!("Waiting for Onebot runtime event loop...");
        let event = self.receiver.as_mut().unwrap().recv().await?;
        log::debug!("Received Onebot event of type [{}].", event.get_type());
        Ok(RuntimeStatus::Event(event))
    }
}
