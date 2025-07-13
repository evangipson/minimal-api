pub mod log_color;

pub mod log_severity;

use crate::{log_color::LogColor, log_severity::LogSeverity};
use std::sync::OnceLock;

static SEVERITY: OnceLock<LogSeverity> = OnceLock::new();

pub fn set_logging_severity(severity: LogSeverity) -> bool {
    SEVERITY.set(severity).is_ok()
}

pub fn log(message: String, severity: &LogSeverity) {
    if severity >= get_logging_severity() {
        println!(
            "{}{: <7}{} {message}",
            severity.get_color().to_string(),
            "[".to_string() + &severity.to_string() + "]",
            LogColor::White.to_string()
        );
    }
}

#[macro_export]
macro_rules! log_debug {
    (message: &str) => {
        $crate::log(message.to_string(), &$crate::log_severity::LogSeverity::Debug);
    };
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*), &$crate::log_severity::LogSeverity::Debug);
    }};
}

#[macro_export]
macro_rules! log_info {
    (message: &str) => {
        $crate::log(message.to_string(), &$crate::log_severity::LogSeverity::Info);
    };
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*), &$crate::log_severity::LogSeverity::Info);
    }};
}

#[macro_export]
macro_rules! log_warning {
    (message: &str) => {
        $crate::log(message.to_string(), &$crate::log_severity::LogSeverity::Warning);
    };
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*), &$crate::log_severity::LogSeverity::Warning);
    }};
}

#[macro_export]
macro_rules! log_error {
    (message: &str) => {
        $crate::log(message.to_string(), &$crate::log_severity::LogSeverity::Error);
    };
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*), &$crate::log_severity::LogSeverity::Error);
    }};
}

fn get_logging_severity() -> &'static LogSeverity {
    SEVERITY.get_or_init(|| LogSeverity::Info)
}
