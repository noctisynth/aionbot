use core::fmt;

use serde_json::Value;

pub struct MessageSegment {
    pub text: String,
    pub type_: String,
}

pub struct Message {
    pub entity: Option<String>,
    pub segments: Vec<MessageSegment>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for segment in &self.segments {
            f.write_str(&segment.text)?;
        }
        Ok(())
    }
}

pub struct Event {
    pub plain_data: Message,
    pub user_id: String,
    pub channel_id: String,
    pub timestamp: String,
    pub event_type: String,
    pub variables: Option<Value>,
}
