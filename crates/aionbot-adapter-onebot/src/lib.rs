use anyhow::Result;

pub extern crate aionbot_core;

pub trait Adapter {
    fn reply(&self, message: &str) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Adapter for aionbot_core::event::Event {
    async fn reply(&self, message: &str) -> Result<()> {
        let _ = message;
        // let ws = onebot_v11::connect::ws_reverse::ReverseWsConnect::new(config);
        unimplemented!()
    }
}
