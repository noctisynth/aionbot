use std::sync::Arc;

use anyhow::Result;
use futures::future::BoxFuture;

use crate::event::Event;

pub type HandlerCallback = BoxFuture<'static, Result<()>>;
pub type Callback = fn(Arc<Event>) -> HandlerCallback;
