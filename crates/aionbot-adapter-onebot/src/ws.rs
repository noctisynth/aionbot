use std::{
    cell::UnsafeCell,
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, Mutex},
    task::JoinHandle,
};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{
        handshake::server::{Request, Response},
        Message,
    },
    WebSocketStream,
};

use crate::{
    event::OnebotEvent,
    models::{Action, ActionParams, MessageEvent},
};

pub struct Config {
    host: String,
    port: u16,
    pub path: String,
    pub access_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            path: "/onebot/v11".to_string(),
            access_token: None,
        }
    }
}

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
                        let plain_data: MessageEvent = match serde_json::from_str(&message) {
                            Ok(data) => data,
                            Err(e) => {
                                log::warn!("Error deserializing message: {}", e);
                                return;
                            }
                        };
                        let event = OnebotEvent {
                            plain_data: plain_data.clone(),
                            bot: self.clone(),
                        };
                        if let Err(e) = bot.sender.send(Box::new(event)) {
                            log::warn!("Error sending event: {}", e);
                        }
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

pub struct Onebot {
    sender: broadcast::Sender<Box<OnebotEvent>>,
    listen_handle: Mutex<Option<JoinHandle<Result<()>>>>,
    bots: RwLock<HashMap<String, Arc<Bot>>>,
}

impl Default for Onebot {
    fn default() -> Self {
        let (tx, _) = broadcast::channel::<Box<OnebotEvent>>(1024);
        Self {
            sender: tx,
            listen_handle: Mutex::new(None),
            bots: Default::default(),
        }
    }
}

impl Onebot {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn listen(self: Arc<Self>, config: Config) -> Result<Arc<Self>> {
        let onebot = self.clone();

        let bind_addr = format!("{}:{}", config.host, config.port);
        log::debug!("Trying to bind on {}.", bind_addr);
        let tcp_listener = match TcpListener::bind(&bind_addr).await {
            Ok(listener) => listener,
            Err(e) => {
                log::error!("Error binding on {}: {}", bind_addr, e);
                return Err(e.into());
            }
        };
        log::info!("Listening on {}.", bind_addr);

        self.listen_handle
            .lock()
            .await
            .replace(tokio::spawn(async move {
                while let Ok((stream, _)) = tcp_listener.accept().await {
                    let bot = Bot::new(onebot.sender.clone());
                    let ws_stream =
                        accept_hdr_async(stream, |req: &Request, response: Response| {
                            let headers = req.headers();
                            let bot_id = headers
                                .get("X-Self-ID")
                                .map(|id| id.to_str().unwrap().to_string())
                                .unwrap_or_default();
                            log::info!("New bot connected with ID: {}.", bot_id);
                            bot.set_id(bot_id.to_string());
                            onebot.bots.write().unwrap().insert(bot_id, bot.clone());
                            Ok(response)
                        })
                        .await?;
                    bot.set_ws_stream(ws_stream);
                    bot.listen().await;
                }
                Ok(())
            }));
        Ok(self)
    }

    pub async fn subscribe(self: Arc<Self>) -> broadcast::Receiver<Box<OnebotEvent>> {
        self.sender.subscribe()
    }

    pub async fn close(&mut self) {
        if let Some(handle) = self.listen_handle.lock().await.take() {
            handle.abort();
        }
    }
}
