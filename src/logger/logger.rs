use crate::conf::conf::Logger;
use fern::Dispatch;
use log::{debug, error, info, trace, warn, LevelFilter, Level};
// Assuming conf module is in the crate
use colored::*;

pub fn setup_logger(config: &Logger) -> Result<(), fern::InitError> {


    // Determine the log level from the configuration
    let log_level = match config.log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info, // Default level
    };

    // Configure base logging
    // Configure base logging
    let base_config = Dispatch::new()
        .level(log_level)
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            let color = match record.level() {
                Level::Error => "red",
                Level::Warn => "yellow",
                Level::Info => "green",
                Level::Debug => "blue",
                Level::Trace => "magenta",
            };
            out.finish(format_args!(
                "{}[{}][{}] {}",
                message.to_string().color(color),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                message
            ))
        });


    let file_config = fern::Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(fern::log_file("logs/log.log")?);


    // Determine output method
    match config.log_output.as_str() {
        "cli" => base_config.apply()?,
        "file" => {
            std::fs::create_dir_all("logs")?;
            file_config.apply()?
        }
        "all" => {
            std::fs::create_dir_all("logs")?;
            base_config.chain(fern::log_file("logs/log.log")?).apply()?
        }
        _ => (),
    }

    Ok(())
}
