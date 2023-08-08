//! logger script, 
//! provide macros to log informations (basically training process) into logfile.

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

use crate::consts::LOG_PATH;

pub fn log_to_file(level: &str, message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    writeln!(file, "[{} - {}] {}", timestamp, level, message).unwrap();
}

#[macro_export]
macro_rules! logger_info {
    ($($arg:tt)*) => {
        crate::logger::log_to_file("INFO", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! logger_warn {
    ($($arg:tt)*) => {
        crate::logger::log_to_file("WARN", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! logger_error {
    ($($arg:tt)*) => {
        crate::logger::log_to_file("ERROR", &format!($($arg)*));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_logger() {
        logger_info!("This is an info message.");
        logger_warn!("This is a warning with number: {}", 404);
        logger_error!("An error occurred!");
    }
}