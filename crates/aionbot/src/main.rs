use std::sync::Arc;

use aionbot_adapter_onebot::Adapter;
use aionbot_macros::register;

#[register(router = ExactMatchRouter::new("hello", false))]
pub async fn hello_world(event: Arc<Event>) -> Result<String> {
    println!("{}", &event.plain_data.to_string());
    event.reply("Hello, world!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let entries = vec![hello_world()];
    for entry in entries {
        println!("{}", entry.id);
    }
}
