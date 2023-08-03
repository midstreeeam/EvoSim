use std::fs::{File, OpenOptions};
use std::io::Write;
use chrono::{Utc, DateTime};

pub struct SimpleLogger {
    file: File,
}

impl SimpleLogger {
    pub fn new(file_name: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)
            .unwrap();

        SimpleLogger { file }
    }

    fn log(&mut self, level: &str, msg: &str) {
        let now: DateTime<Utc> = Utc::now();
        let log_msg = format!("{} [{}] {}\n", now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true), level, msg);

        self.file.write_all(log_msg.as_bytes()).unwrap();
    }
}

lazy_static::lazy_static! {
    static ref LOGGER: std::sync::Mutex<SimpleLogger> = std::sync::Mutex::new(SimpleLogger::new("training.log"));
}

#[macro_export]
macro_rules! logger_info {
    ($($arg:tt)*) => {
        LOGGER.lock().unwrap().log("INFO", &format_args!($($arg)*).to_string());
    };
}

#[macro_export]
macro_rules! logger_warn {
    ($($arg:tt)*) => {
        LOGGER.lock().unwrap().log("WARN", &format_args!($($arg)*).to_string());
    };
}

#[macro_export]
macro_rules! logger_error {
    ($($arg:tt)*) => {
        LOGGER.lock().unwrap().log("ERROR", &format_args!($($arg)*).to_string());
    };
}
