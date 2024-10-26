use std::sync::Arc;

use aionbot_core::event::Event;

use crate::{models::MessageEvent, ws::Bot};

#[derive(Clone, Debug)]
pub struct OnebotEvent {
    pub plain_data: MessageEvent,
    pub bot: Arc<Bot>,
}

impl Event for OnebotEvent {
    fn get_type(&self) -> &str {
        &self.plain_data.message_type
    }

    fn get_content(&self) -> Box<dyn std::any::Any> {
        let content = self
            .plain_data
            .message
            .iter()
            .map(|segment| segment.data.text.clone())
            .collect::<Vec<String>>();
        let result: &str = content.join("").leak();
        Box::new(result)
    }

    fn get_plain_data(&self) -> Box<dyn std::any::Any> {
        Box::new(self.plain_data.clone())
    }

    fn get_emitter_id(&self) -> &str {
        self.plain_data.user_id.to_string().leak()
    }

    fn get_channel_id(&self) -> &str {
        self.plain_data
            .group_id
            .expect("Channel ID is not set, this event is most likely a private message")
            .to_string()
            .leak()
    }
}

impl OnebotEvent {
    pub fn is_private(&self) -> bool {
        self.plain_data.message_type == "private"
    }
}
