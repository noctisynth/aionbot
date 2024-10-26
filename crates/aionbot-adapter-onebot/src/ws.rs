use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::Result;
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
    task::JoinHandle,
};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};

use crate::{bot::Bot, event::OnebotEvent};

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
