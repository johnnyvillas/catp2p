/* Copyright 2025 Joao Guimaraes, Catp2p Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Logging utilities.

use crate::error::Error;
use log::{LevelFilter, info, warn, error, debug, trace};
use std::path::Path;

/// Initializes the logger.
pub fn init_logger(level: LevelFilter) -> Result<(), Error> {
    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp_millis()
        .init();
    
    info!("Logger initialized with level: {:?}", level);
    
    Ok(())
}

/// Initializes the logger with a file output.
pub fn init_file_logger<P: AsRef<Path>>(level: LevelFilter, path: P) -> Result<(), Error> {
    use std::fs::File;
    use std::io::Write;
    
    let file = File::create(path).map_err(|e| {
        Error::Logging(format!("Failed to create log file: {}", e))
    })?;
    
    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp_millis()
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
    
    info!("File logger initialized with level: {:?}", level);
    
    Ok(())
}

/// Logs a message at the info level.
pub fn log_info(message: &str) {
    info!("{}", message);
}

/// Logs a message at the warn level.
pub fn log_warn(message: &str) {
    warn!("{}", message);
}

/// Logs a message at the error level.
pub fn log_error(message: &str) {
    error!("{}", message);
}

/// Logs a message at the debug level.
pub fn log_debug(message: &str) {
    debug!("{}", message);
}

/// Logs a message at the trace level.
pub fn log_trace(message: &str) {
    trace!("{}", message);
}
