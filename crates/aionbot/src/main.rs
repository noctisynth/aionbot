use std::sync::Arc;

use aionbot_adapter_onebot::{Adapter, OnebotRuntime};
use aionbot_macros::register;

struct State {}

#[register(router = ExactMatchRouter::new("/hello"))]
pub async fn hello_world(event: Arc<Box<dyn Event>>) -> Result<String> {
    // println!("{}", &event.get_plain_data());
    println!(
        "Event content: {}",
        event.get_content().downcast::<&str>().unwrap()
    );
    event.reply("Hello, world!").await?;
    Ok(())
}

#[register(router = StartsWithRouter::new("/bot"))]
pub async fn bot_info(event: Arc<Box<dyn Event>>) -> Result<String> {
    event
        .reply(
            r#"NoctisBot v0.1.0-alpha.0 [Rust edition 2024 for AionBot v0.1.0-alpha.0]
This project is open-source and licensed under the NOL (Normalcy Open License).
Type [.help] to show more information."#,
            //             r#"NoctisBot v0.1.0-alpha.0 [Rust v1.83.0-nightly for Aionbot v0.1.0-alpha.0]
            // 此项目以常态开源协议(NOL)开放许可证的形式开源.
            // 输入[.help]以获取更多帮助信息."#,
        )
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    aionbot_core::runtime::Builder::<OnebotRuntime>::default()
        .invoke_handler(vec![hello_world(), bot_info()])
        .manage(State {})
        .run()
        .await
        .expect("Failed to start the bot runtime.");
}
