use std::any::Any;

use anyhow::{anyhow, Result};

pub trait Event: Any + Send + Sync {
    /// Get the name of the event.
    fn name(&self) -> &str {
        "UnknownEvent"
    }
    /// Get the type of the event.
    fn event_type(&self) -> &str;
    /// Get the deserialized content of the event.
    fn content(&self) -> Box<dyn Any> {
        unimplemented!()
    }
    /// Get the plain data of the event.
    fn plain_data(&self) -> Box<dyn Any> {
        unimplemented!()
    }
    /// Get the emitter ID of the event.
    fn emitter_id(&self) -> &str {
        unimplemented!()
    }
    /// Get the channel ID of the event.
    fn channel_id(&self) -> Result<&str> {
        unimplemented!()
    }
    /// Get the plain text of the event.
    fn plain_text(&self) -> Result<&str> {
        if let Ok(plain_text) = self.content().downcast::<&str>() {
            Ok(*plain_text)
        } else {
            Err(anyhow!(
                "Failed to downcast plain data to [&str], \
            this is most likely a AionBot internal bug."
            ))
        }
    }
}
