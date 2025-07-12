use std::sync::OnceLock;

#[derive(PartialEq, PartialOrd)]
pub enum LogSeverity {
    Debug,
    Info,
    Warning,
    Error,
}

impl ToString for LogSeverity {
    fn to_string(&self) -> String {
        match self {
            LogSeverity::Debug => "debug".to_owned(),
            LogSeverity::Info => "info".to_owned(),
            LogSeverity::Warning => "warn".to_owned(),
            LogSeverity::Error => "error".to_owned(),
        }
    }
}

impl LogSeverity {
    pub fn get_color(&self) -> LogColor {
        match self {
            LogSeverity::Debug => LogColor::BrightGreen,
            LogSeverity::Info => LogColor::BrightBlue,
            LogSeverity::Warning => LogColor::BrightYellow,
            LogSeverity::Error => LogColor::BrightRed,
        }
    }
}

pub const RESET_COLOR: &str = "\x1b";

pub enum LogColor {
    BrightBlue,
    BrightGreen,
    BrightRed,
    BrightYellow,
    Blue,
    Green,
    Red,
    Yellow,
    White,
}

impl ToString for LogColor {
    fn to_string(&self) -> String {
        match self {
            LogColor::Green => format!("{RESET_COLOR}[32m"),
            LogColor::BrightGreen => format!("{RESET_COLOR}[32;1m"),
            LogColor::Blue => format!("{RESET_COLOR}[34m"),
            LogColor::BrightBlue => format!("{RESET_COLOR}[34;1m"),
            LogColor::Yellow => format!("{RESET_COLOR}[33m"),
            LogColor::BrightYellow => format!("{RESET_COLOR}[33;1m"),
            LogColor::Red => format!("{RESET_COLOR}[31m"),
            LogColor::BrightRed => format!("{RESET_COLOR}[31;1m"),
            LogColor::White => format!("{RESET_COLOR}[37;0m"),
        }
    }
}

static SEVERITY: OnceLock<LogSeverity> = OnceLock::new();
pub fn get_logging_severity() -> &'static LogSeverity {
    SEVERITY.get_or_init(|| LogSeverity::Info)
}
pub fn set_logging_severity(severity: LogSeverity) -> bool {
    SEVERITY.set(severity).is_ok()
}

#[macro_export]
macro_rules! log_debug {
    (message: &str) => {
        $crate::logger::log(message.to_string(), &$crate::logger::LogSeverity::Debug);
    };
    ($($arg:tt)*) => {{
        $crate::logger::log(format!($($arg)*), &$crate::logger::LogSeverity::Debug);
    }};
}

#[macro_export]
macro_rules! log_info {
    (message: &str) => {
        $crate::logger::log(message.to_string(), &$crate::logger::LogSeverity::Info);
    };
    ($($arg:tt)*) => {{
        $crate::logger::log(format!($($arg)*), &$crate::logger::LogSeverity::Info);
    }};
}

#[macro_export]
macro_rules! log_warning {
    (message: &str) => {
        $crate::logger::log(message.to_string(), &$crate::logger::LogSeverity::Warning);
    };
    ($($arg:tt)*) => {{
        $crate::logger::log(format!($($arg)*), &$crate::logger::LogSeverity::Warning);
    }};
}

#[macro_export]
macro_rules! log_error {
    (message: &str) => {
        $crate::logger::log(message.to_string(), &$crate::logger::LogSeverity::Error);
    };
    ($($arg:tt)*) => {{
        $crate::logger::log(format!($($arg)*), &$crate::logger::LogSeverity::Error);
    }};
}

pub fn log(message: String, severity: &LogSeverity) {
    if get_logging_severity() >= severity {
        println!(
            "{}{: <7}{} {message}",
            severity.get_color().to_string(),
            "[".to_string() + &severity.to_string() + "]",
            LogColor::White.to_string()
        );
    }
}
