use crate::log_color::LogColor;

#[derive(PartialEq, PartialOrd)]
pub enum LogSeverity {
    Debug,
    Info,
    Warning,
    Error,
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
