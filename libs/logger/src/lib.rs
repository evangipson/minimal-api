//! # [`logger`](crate)
//! The [`logger`](crate) crate contains macros to make it easy to log messages to the
//! console of varying [`LogSeverity`].
//!
//! Make sure to set the logger severity with [`set_logging_severity`] if you'd like to
//! hide or show log messages based on the macros that are called to log any messages.
//!
//! It provides helpful `#[macro_export]` macros to enable logging throughout an application:
//! - [`log_debug`](macro@log_debug)
//! - [`log_info`](macro@log_info)
//! - [`log_warning`](macro@log_warning)
//! - [`log_error`](macro@log_error)
//!
//! # Examples
//! ## Basic logging
//! [`log_info`](macro@log_info) can be used to log out an informational message:
//! ```rust
//! use logger::log_info;
//!
//! fn sum(a: i32, b: i32) -> i32 {
//!     log_info!("running the sum function: {} + {} = {}", a, b, a + b);
//!     a + b
//! }
//! ```
//!
//! ## Setting logger severity
//! [`set_logging_severity`] can be used to filter out all debugging log messages:
//! ```rust
//! use logger::{
//!     self,
//!     log_debug,
//!     log_severity::LogSeverity,
//! };
//!
//! fn sum(a: i32, b: i32) -> i32 {
//!     // filter out all log messages less severe than info (i.e.: debug logs)
//!     logger::set_logging_severity(LogSeverity::Info);
//!
//!     // as a result, this debug log will not appear
//!     log_debug!("calling sum function");
//!
//!     a + b
//! }
//! ```

pub mod log_color;
pub mod log_severity;
use crate::{log_color::LogColor, log_severity::LogSeverity};
use std::sync::OnceLock;

/// [`SEVERITY`] is a `static` [`LogSeverity`] that is initialized once in a
/// thread-safe manner.
static SEVERITY: OnceLock<LogSeverity> = OnceLock::new();
fn get_logging_severity() -> &'static LogSeverity {
    SEVERITY.get_or_init(|| LogSeverity::Info)
}

#[doc = r#"
# log_debug
The [`log_debug`](macro@log_debug) macro logs a message as debugging information
to the console, as long as [`set_logging_severity`] has been used to set the logger's
severity to [`LogSeverity::Debug`].

# Example
[`log_debug`](macro@log_debug) can be used to log out a debug message:
```rust
use logger::log_debug;

fn sum(a: i32, b: i32) -> i32 {
    log_debug!("running the sum function: {} + {} = {}", a, b, a + b);
    a + b
}
```
"#]
#[macro_export]
macro_rules! log_debug {
    (message: &str) => {
        $crate::log(message.to_string(), &$crate::log_severity::LogSeverity::Debug);
    };
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*), &$crate::log_severity::LogSeverity::Debug);
    }};
}

#[doc = r#"
# log_info
The [`log_info`](macro@log_info) macro logs an informational message to the console,
as long as [`set_logging_severity`] has been used to set the logger's severity to
[`LogSeverity::Info`] or anything less severe.

# Example
[`log_info`](macro@log_info) can be used to log out an informational message:
```rust
use logger::log_info;

fn sum(a: i32, b: i32) -> i32 {
    log_info!("running the sum function: {} + {} = {}", a, b, a + b);
    a + b
}
```
"#]
#[macro_export]
macro_rules! log_info {
    (message: &str) => {
        $crate::log(message.to_string(), &$crate::log_severity::LogSeverity::Info);
    };
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*), &$crate::log_severity::LogSeverity::Info);
    }};
}

#[doc = r#"
# log_warning
The [`log_warning`](macro@log_warning) macro logs a warning message to the console,
as long as [`set_logging_severity`] has been used to set the logger's severity to
[`LogSeverity::Warning`] or anything less severe.

# Example
[`log_warning`](macro@log_warning) can be used to log out a warning message:
```rust
use logger::log_warning;

fn sum(a: i32, b: i32) -> i32 {
    log_warning!("running the sum function: {} + {} = {}", a, b, a + b);
    a + b
}
```
"#]
#[macro_export]
macro_rules! log_warning {
    (message: &str) => {
        $crate::log(message.to_string(), &$crate::log_severity::LogSeverity::Warning);
    };
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*), &$crate::log_severity::LogSeverity::Warning);
    }};
}

#[doc = r#"
# log_error
The [`log_error`](macro@log_error) macro logs an error message to the console,
regardless of the logger's severity.

# Example
[`log_error`](macro@log_error) can be used to log out an error message:
```rust
use logger::log_error;

fn sum(a: i32, b: i32) -> i32 {
    log_error!("running the sum function: {} + {} = {}", a, b, a + b);
    a + b
}
```
"#]
#[macro_export]
macro_rules! log_error {
    (message: &str) => {
        $crate::log(message.to_string(), &$crate::log_severity::LogSeverity::Error);
    };
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*), &$crate::log_severity::LogSeverity::Error);
    }};
}

/// [`log`] will log a message to the console, provided the `severity` is greater
/// than or equal to the severity that has been set with [`set_logging_severity`].
///
/// Using the macros such as [`log_info`](macro@log_info) is preferable and provides
/// a better experience.
///
/// # Example
/// [`log`] can be used to write a message of any severity to the console:
/// ```rust
/// use logger::{self, log_severity::LogSeverity};
///
/// fn sum(a: i32, b: i32) -> i32 {
///     logger::log("running the sum function.".to_string(), &LogSeverity::Info);
///     a + b
/// }
/// ```
pub fn log(message: String, severity: &LogSeverity) {
    if severity >= get_logging_severity() {
        println!(
            "{}{: <7}{} {message}{}",
            severity.get_color().to_string(),
            "[".to_string() + &severity.to_string() + "]",
            LogColor::Grey.to_string(),
            LogColor::White.to_string()
        );
    }
}

/// [`set_logging_severity`] will set the logger's global severity to the provided
/// `severity`, to filter log messages to only show messages that are **at least**
/// that severe.
///
/// # Example
/// [`set_logging_severity`] can be used to filter out all debugging log messages:
/// ```rust
/// use logger::{
///     self,
///     log_debug,
///     log_severity::LogSeverity,
/// };
///
/// fn sum(a: i32, b: i32) -> i32 {
///     // filter out all log messages less severe than info (i.e.: debug logs)
///     logger::set_logging_severity(LogSeverity::Info);
///
///     // as a result, this debug log will not appear
///     log_debug!("calling sum function");
///
///     a + b
/// }
/// ```
pub fn set_logging_severity(severity: LogSeverity) -> bool {
    SEVERITY.set(severity).is_ok()
}
