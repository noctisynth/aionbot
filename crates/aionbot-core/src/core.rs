use std::sync::Arc;

use anyhow::Result;

use crate::{event::Event, handler::Handler};

pub struct Core {
    handlers: Vec<Handler>,
}

impl Core {
    pub async fn input(&mut self, event: Arc<Event>) -> Result<()> {
        for handler in self.handlers.iter() {
            handler.input(&event).await?;
        }
        Ok(())
    }
}
