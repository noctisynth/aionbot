use std::sync::Arc;

use aionbot_adapter_onebot::{Adapter, OnebotRuntime};
use aionbot_macros::register;
use anyhow::Result;

use colored::Colorize;

struct State {}

#[register(router = ExactMatchRouter::new("/hello"))]
pub async fn hello_world(event: Arc<Box<dyn Event>>) -> Result<String> {
    println!(
        "Event content: {}",
        event.content().downcast::<&str>().unwrap()
    );
    event.reply("Hello, world!").await?;
    Ok(())
}

fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let level = record.level().as_str();
            let module = record.target().split("::").next().unwrap_or_default();
            let message = message.to_string();
            let fmt_string = match record.level() {
                log::Level::Error => {
                    format!("{:6}| {} - {}", level.red(), module.green(), message.red())
                }
                log::Level::Warn => format!(
                    "{:6}| {} - {}",
                    level.bright_yellow(),
                    module.green(),
                    message.bright_yellow()
                ),
                log::Level::Info => format!(
                    "{:6}| {} - {}",
                    level.blue(),
                    module.green(),
                    message.blue()
                ),
                log::Level::Debug => {
                    format!(
                        "{:6}| {} - {}",
                        level.magenta(),
                        module.green(),
                        message.magenta()
                    )
                }
                log::Level::Trace => {
                    format!("{:6}| {}", level.bright_black(), message.bright_black())
                }
            };
            out.finish(format_args!("{}", fmt_string))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;

    aionbot_core::runtime::Builder::<OnebotRuntime>::default()
        .invoke_handler(vec![hello_world()])
        .manage(State {})
        .run()
        .await
        .expect("Failed to start the bot runtime.");

    Ok(())
}
