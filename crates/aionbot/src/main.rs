use std::sync::Arc;

use aionbot_adapter_onebot::{Adapter, OnebotRuntime};
use aionbot_macros::register;

struct State {}

#[register(router = ExactMatchRouter::new("hello", false))]
pub async fn hello_world(event: Arc<Event>) -> Result<String> {
    println!("{}", &event.plain_data.to_string());
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
