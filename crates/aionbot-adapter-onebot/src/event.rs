use aionbot_core::event::Event;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct OnebotEvent {
    pub event_type: String,
    pub message: String,
    pub plain_data: Value,
    pub emitter_id: String,
    pub channel_id: String,
}

impl Event for OnebotEvent {
    fn get_type(&self) -> &str {
        &self.event_type
    }

    fn get_content(&self) -> Box<dyn std::any::Any> {
        Box::new(self.message.clone())
    }

    fn get_plain_data(&self) -> Box<dyn std::any::Any> {
        Box::new(self.plain_data.clone())
    }

    fn get_emitter_id(&self) -> &str {
        &self.emitter_id
    }

    fn get_channel_id(&self) -> &str {
        &self.channel_id
    }
}
