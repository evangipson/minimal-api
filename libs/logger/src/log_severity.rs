use crate::log_color::LogColor;

/// [`LogSeverity`] represents different levels of logging severity.
#[derive(PartialEq, PartialOrd)]
pub enum LogSeverity {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogSeverity {
    /// [`LogSeverity::get_color`] returns a [`LogColor`] for the
    /// [`LogSeverity`] that calls it.
    ///
    /// # Example
    /// [`LogSeverity::get_color`] can be used to get the [`LogColor`]
    /// associated to any [`LogSeverity`]:
    /// ```rust
    /// use logger::{
    ///     log_color::LogColor,
    ///     log_severity::LogSeverity,
    /// };
    ///
    /// fn get_error_color() -> LogColor {
    ///     LogSeverity::Error.get_color()
    /// }
    /// ```
    pub fn get_color(&self) -> LogColor {
        match self {
            LogSeverity::Debug => LogColor::BrightGreen,
            LogSeverity::Info => LogColor::BrightBlue,
            LogSeverity::Warning => LogColor::BrightYellow,
            LogSeverity::Error => LogColor::BrightRed,
        }
    }
}

/// Implement [`ToString`] for [`LogSeverity`].
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
