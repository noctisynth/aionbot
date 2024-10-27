use std::sync::Arc;

use aionbot_core::event::Event;
use anyhow::{anyhow, Result};

use crate::{bot::Bot, models::MessageEvent};

#[derive(Clone, Debug)]
pub struct OnebotEvent {
    pub plain_data: MessageEvent,
    pub bot: Arc<Bot>,
}

impl Event for OnebotEvent {
    fn event_type(&self) -> &str {
        &self.plain_data.message_type
    }

    fn content(&self) -> Box<dyn std::any::Any> {
        let content = self
            .plain_data
            .message
            .iter()
            .map(|segment| segment.data.text.clone())
            .collect::<Vec<String>>();
        let result: &str = content.join("").leak();
        Box::new(result)
    }

    fn plain_data(&self) -> Box<dyn std::any::Any> {
        Box::new(self.plain_data.clone())
    }

    fn emitter_id(&self) -> &str {
        self.plain_data.user_id.to_string().leak()
    }

    fn channel_id(&self) -> Result<&str> {
        if let Some(group_id) = self.plain_data.group_id {
            Ok(group_id.to_string().leak())
        } else {
            Err(anyhow!(
                "Group ID not found in this event, \
            perhaps this is not message from channel?"
            ))
        }
    }

    fn reply<'s, 'a>(
        &'s self,
        message: Box<dyn ToString + Send + Sync>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>>
    where
        's: 'a,
    {
        let bot = self.bot.clone();
        Box::pin(async move {
            let message = message.to_string();
            bot.send(&self, &message).await;
            Ok(())
        })
    }
}

impl OnebotEvent {
    pub fn is_private(&self) -> bool {
        self.plain_data.message_type == "private"
    }
}
