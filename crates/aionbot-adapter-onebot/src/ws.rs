use std::sync::Arc;

use anyhow::Result;
use futures_util::StreamExt;
use tokio::{net::TcpListener, sync::Mutex};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};

pub struct Config {
    host: String,
    port: u16,
    path: String,
    access_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 6700,
            path: "/onebot".to_string(),
            access_token: None,
        }
    }
}

#[derive(Default)]
pub struct Onebot {
    listen_handle: Mutex<Option<tokio::task::JoinHandle<Result<()>>>>,
    bot_handles: Mutex<Vec<tokio::task::JoinHandle<Result<()>>>>,
}

impl Onebot {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn listen(self: Arc<Self>, config: Config) -> Result<Arc<Self>> {
        let onebot = self.clone();

        let tcp_listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

        self.listen_handle
            .lock()
            .await
            .replace(tokio::spawn(async move {
                while let Ok((stream, _)) = tcp_listener.accept().await {
                    onebot.bot_handles.lock().await.push(tokio::spawn(async {
                        let ws_stream =
                            accept_hdr_async(stream, |req: &Request, response: Response| {
                                let headers = req.headers();
                                let bot_id = headers
                                    .get("X-Self-ID")
                                    .map(|id| id.to_str().unwrap().to_string())
                                    .unwrap();
                                Ok(response)
                            })
                            .await?;
                        ws_stream.for_each(|message| async {}).await;
                        Ok(())
                    }));
                }
                Ok(())
            }));
        Ok(self)
    }

    // pub async fn close(&mut self) {
    //     if let Some(handle) = self.listen_handle.take() {
    //         handle.abort();
    //     }
    // }
}
