//! Time utilities.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Gets the current timestamp in seconds.
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

/// Gets the current timestamp in milliseconds.
pub fn current_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_millis(0))
        .as_millis()
}

/// Formats a timestamp as a human-readable string.
pub fn format_timestamp(timestamp: u64) -> String {
    let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now());
    
    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Formats a duration as a human-readable string.
pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    
    if seconds < 60 {
        format!("{} seconds", seconds)
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;
        format!("{} minutes, {} seconds", minutes, remaining_seconds)
    } else if seconds < 86400 {
        let hours = seconds / 3600;
        let remaining_minutes = (seconds % 3600) / 60;
        format!("{} hours, {} minutes", hours, remaining_minutes)
    } else {
        let days = seconds / 86400;
        let remaining_hours = (seconds % 86400) / 3600;
        format!("{} days, {} hours", days, remaining_hours)
    }
}

/// Parses a timestamp from a string.
pub fn parse_timestamp(timestamp_str: &str) -> Option<u64> {
    let datetime = chrono::DateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S %z");
    
    match datetime {
        Ok(dt) => Some(dt.timestamp() as u64),
        Err(_) => None,
    }
}
