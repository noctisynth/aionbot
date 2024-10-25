use std::any::Any;

pub trait Event: Any + Send + Sync {
    fn get_plain_data(&self) -> Box<dyn Any>;
    fn get_emitter_id(&self) -> &str;
    fn get_channel_id(&self) -> &str;
}
