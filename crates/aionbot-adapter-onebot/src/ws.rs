use std::sync::Arc;

use anyhow::Result;
use futures_util::StreamExt;
use tokio::{net::TcpListener, sync::{broadcast, Mutex}};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};

use crate::event::OnebotEvent;

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
    sender: tokio::sync::broadcast::Sender<Box<OnebotEvent>>,
    listen_handle: Mutex<Option<tokio::task::JoinHandle<Result<()>>>>,
    bot_handles: Mutex<Vec<tokio::task::JoinHandle<Result<()>>>>,
}

impl Default for Onebot {
    fn default() -> Self {
        let (tx, _) = broadcast::channel::<Box<OnebotEvent>>(1024);
        Self {
            sender: tx,
            listen_handle: Mutex::new(None),
            bot_handles: Mutex::new(vec![]),
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
        let tcp_listener = TcpListener::bind(&bind_addr).await?;
        println!("Listening on {}", bind_addr);

        self.listen_handle
            .lock()
            .await
            .replace(tokio::spawn(async move {
                println!("Starting bot listening loop");
                while let Ok((stream, _)) = tcp_listener.accept().await {
                    println!("New connection found.");
                    let sender = onebot.sender.clone();
                    onebot
                        .bot_handles
                        .lock()
                        .await
                        .push(tokio::spawn(async move {
                            let ws_stream =
                                accept_hdr_async(stream, |req: &Request, response: Response| {
                                    let headers = req.headers();
                                    let bot_id = headers
                                        .get("X-Self-ID")
                                        .map(|id| id.to_str().unwrap().to_string())
                                        .unwrap_or_default();
                                    println!("New bot connection: {}", bot_id);
                                    sender.send(Default::default()).unwrap();
                                    Ok(response)
                                })
                                .await?;
                            ws_stream
                                .for_each(|message| {
                                    let value = sender.clone();
                                    async move {
                                        println!("Received message: {:?}", &message);
                                        value.send(Default::default()).unwrap();
                                    }
                                })
                                .await;
                            Ok(())
                        }));
                }
                Ok(())
            }));
        Ok(self)
    }

    pub async fn subscribe(self: Arc<Self>) -> broadcast::Receiver<Box<OnebotEvent>> {
        self.sender.subscribe()
    }

    // pub async fn close(&mut self) {
    //     if let Some(handle) = self.listen_handle.take() {
    //         handle.abort();
    //     }
    // }
}
