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
    access_token: String,
}

#[derive(Default)]
pub struct Onebot {
    listen_handle: Mutex<Option<tokio::task::JoinHandle<Result<()>>>>,
    bot_handles: Mutex<Vec<tokio::task::JoinHandle<Result<()>>>>,
    // stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>,
}

impl Onebot {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn listen(self: Arc<Self>, config: Config) -> Result<()> {
        let onebot = self.clone();

        let tcp_listener = TcpListener::bind(&config.host).await?;

        self.listen_handle
            .lock()
            .await
            .replace(tokio::spawn(async move {
                let url = format!(
                    "{}:{}/{}",
                    config.host,
                    config.port,
                    config.path.trim_start_matches("/")
                );
                while let Ok((stream, _)) = tcp_listener.accept().await {
                    // onebot.bot_handles.lock().await.push(
                    onebot.bot_handles.lock().await.push(tokio::spawn(async {
                        stream.set_nodelay(true).unwrap(); // TODO: No delay?
                        let ws_stream =
                            accept_hdr_async(stream, |req: &Request, response: Response| {
                                req.headers().iter();Ok(response.clone())
                            })
                            .await?;
                        ws_stream.for_each(|message| async {}).await;
                        Ok(())
                    }));
                }
                Ok(())
            }));
        Ok(())
    }

    // pub async fn close(&mut self) {
    //     if let Some(handle) = self.listen_handle.take() {
    //         handle.abort();
    //     }
    // }
}
