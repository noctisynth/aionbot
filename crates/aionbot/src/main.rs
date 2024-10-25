use std::sync::Arc;

use aionbot_adapter_onebot::{Adapter, OnebotRuntime};
use aionbot_macros::register;

struct State {}

#[register(router = ExactMatchRouter::new("hello"))]
pub async fn hello_world(event: Arc<Box<dyn Event>>) -> Result<String> {
    // println!("{}", &event.get_plain_data());
    event.reply("Hello, world!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    aionbot_core::runtime::Builder::<OnebotRuntime>::default()
        .invoke_handler(vec![hello_world()])
        .manage(State {})
        .run()
        .await
        .expect("Failed to start the bot runtime.");
}
