use log::{error, warn, info, debug, trace, LevelFilter};
use std::fs::File;
use fern::Dispatch;
use crate::conf::conf::Logger; // Assuming conf module is in the crate

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
    let mut base_config = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}: {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level);

    // Determine output method
    match config.output.to_lowercase().as_str() {
        "file" => {
            base_config = base_config.chain(File::create(&config.log_file_path)?);
        },
        "console" => {
            base_config = base_config.chain(std::io::stdout());
        },
        "both" => {
            base_config = base_config.chain(std::io::stdout())
                                     .chain(File::create(&config.log_file_path)?);
        },
        _ => {
            base_config = base_config.chain(std::io::stdout()); // Default output
        }
    }

    // Apply the configuration
    base_config.apply()?;

    Ok(())
}

