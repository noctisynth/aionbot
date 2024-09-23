use anyhow::Result;
use futures::future::BoxFuture;

use crate::event::Event;

pub type Callback = fn(&Event) -> BoxFuture<'static, Result<String>>;
