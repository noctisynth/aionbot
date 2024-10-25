use aionbot_core::event::Event;

#[derive(Debug, Clone, Default)]
pub struct OnebotEvent {
    pub message: String,
    pub emitter_id: String,
    pub channel_id: String,
}

impl Event for OnebotEvent {
    fn get_plain_data(&self) -> Box<dyn std::any::Any> {
        Box::new(self.message.clone())
    }

    fn get_emitter_id(&self) -> &str {
        &self.emitter_id
    }

    fn get_channel_id(&self) -> &str {
        &self.channel_id
    }
}