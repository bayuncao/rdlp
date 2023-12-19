
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use env_logger::Builder;

pub fn init() -> Result<(), SetLoggerError> {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    Ok(())
}