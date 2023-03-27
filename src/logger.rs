//! Logging setup.
//!
//! This module contains the setup_logging function, which sets up the logger
//! for the application.
//!

use chrono::Local;
use colored::*;
use log::Level;
use log::LevelFilter;
use std::io;

/// Sets up logging.
///
/// # Arguments
///
/// `log_level` - The log level to use
/// `log_file` - The path to the log file
///
pub fn setup_logging(
    log_level: LevelFilter,
    log_file: Option<String>,
) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new()
        // Default format for log messages.
        .format(move |out, message, record| {
            let color = match record.level() {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Green,
                Level::Debug => Color::Cyan,
                Level::Trace => Color::Blue,
            };
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                format!("{}", record.level()).color(color),
                message
            ))
        })
        .level(log_level);

    // If a log file is provided, log to that file.
    if let Some(log_file) = log_file {
        base_config = base_config.chain(fern::log_file(log_file)?);
    }

    // If the log level is debug, log to stdout as well.
    if log_level == LevelFilter::Debug {
        base_config = base_config.chain(io::stdout());
    }

    // Apply the configuration to the logger.
    base_config.apply()?;

    Ok(())
}
