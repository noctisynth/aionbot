use anyhow::Result;
use colored::Colorize;

pub fn setup_logger(log_level: log::LevelFilter) -> Result<()> {
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
        .level(log_level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
