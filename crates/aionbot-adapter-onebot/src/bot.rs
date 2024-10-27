use std::{cell::UnsafeCell, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpStream, sync::broadcast};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    event::OnebotEvent,
    models::{Action, ActionParams, MessageEvent, MinimalEvent},
};

#[derive(Debug)]
pub struct BotInstance {
    pub id: String,
    pub ws_stream: Option<WebSocketStream<TcpStream>>,
    sender: broadcast::Sender<Box<OnebotEvent>>,
}

#[derive(Debug)]
pub struct Bot {
    inner: UnsafeCell<BotInstance>,
}

unsafe impl Send for Bot {}
unsafe impl Sync for Bot {}

impl Bot {
    pub fn new(sender: broadcast::Sender<Box<OnebotEvent>>) -> Arc<Self> {
        Arc::new(Self {
            inner: UnsafeCell::new(BotInstance {
                id: String::new(),
                ws_stream: None,
                sender,
            }),
        })
    }

    pub fn id(&self) -> &str {
        unsafe { &(*self.inner.get()).id }
    }

    pub fn set_id(&self, id: String) {
        unsafe { (*self.inner.get()).id = id }
    }
    pub fn set_ws_stream(&self, ws_stream: WebSocketStream<TcpStream>) {
        unsafe { (*self.inner.get()).ws_stream = Some(ws_stream) }
    }

    pub async fn listen(self: Arc<Self>) {
        let bot = unsafe { &mut (*self.inner.get()) };
        if let Some(ws_stream) = &mut bot.ws_stream {
            log::info!("Starting listening for messages from bot {}...", bot.id);
            ws_stream
                .for_each(|message| async {
                    if let Ok(Message::Text(message)) = message {
                        log::debug!("Received event message: {}", message);
                        match serde_json::from_str::<MinimalEvent>(&message) {
                            Ok(data) => {
                                if !data.is_message() {
                                    log::debug!(
                                        "Received non-message event: {}, ignored.",
                                        message
                                    );
                                    return;
                                }
                            }
                            Err(e) => {
                                log::warn!("Error deserializing event minimally: {}", e);
                                return;
                            }
                        };
                        let message_event: MessageEvent = match serde_json::from_str(&message) {
                            Ok(data) => data,
                            Err(e) => {
                                log::error!("Error deserializing message: {}", e);
                                return;
                            }
                        };
                        let event = OnebotEvent {
                            plain_data: message_event,
                            bot: self.clone(),
                        };
                        if let Err(e) = bot.sender.send(Box::new(event)) {
                            log::warn!("Error sending event: {}", e);
                        }
                    } else {
                        log::warn!("Received non-text message: {:?}", message)
                    }
                })
                .await;
        };
    }

    pub async fn send(&self, event: &OnebotEvent, message: &str) {
        if let Some(ws_stream) = &mut unsafe { &mut (*self.inner.get()) }.ws_stream {
            ws_stream
                .send(Message::Text(
                    serde_json::to_string(&Action {
                        action: if event.is_private() {
                            "send_private_msg".to_string()
                        } else {
                            "send_group_msg".to_string()
                        },
                        params: ActionParams {
                            group_id: if event.is_private() {
                                None
                            } else {
                                event.plain_data.group_id
                            },
                            user_id: Some(event.plain_data.user_id),
                            message: message.to_string(),
                        },
                        echo: Some("0".to_string()),
                    })
                    .unwrap(),
                ))
                .await
                .unwrap();
        }
    }
}
