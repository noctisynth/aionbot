use std::any::Any;

pub trait Event: Any + Send + Sync {
    /// Get the type of the event.
    fn get_type(&self) -> &str;
    /// Get the deserialized content of the event.
    fn get_content(&self) -> Box<dyn Any>;
    /// Get the plain data of the event.
    ///
    /// Always returns `Box<dyn Any>` casted from `serde_json::Value`.
    fn get_plain_data(&self) -> Box<dyn Any>;
    /// Get the emitter ID of the event.
    fn get_emitter_id(&self) -> &str;
    /// Get the channel ID of the event.
    fn get_channel_id(&self) -> &str;
    fn get_plain_text(&self) -> &str {
        *self.get_content().downcast::<&str>().unwrap()
    }
}
