/// Logs a message with `LogLevel::Debug`.
///
/// This macro allows you to log debug-level messages, which are typically used
/// for development and troubleshooting. It accepts any number of arguments and
/// formats them into a string using the `format!` macro.
///
/// # Example
///
/// ```no_run
/// debug!("Debugging value: {}", value);
/// ```
///
/// This will log the message "Debugging value: {value}" at the debug level.
/// The message is processed by the logging system and sent to the appropriate
/// log targets.
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => ({
        $crate::core::diego_runtime::add_log_message($crate::logging::LogLevel::Debug, format!($($arg)*));
    })
}

/// Logs a message with `LogLevel::Info`.
///
/// This macro is used for logging informational messages that are of general
/// interest but do not represent errors or warnings. It accepts any number of
/// arguments and formats them into a string using the `format!` macro.
///
/// # Example
///
/// ```no_run
/// info!("Application started at: {}", timestamp);
/// ```
///
/// This will log the message "Application started at: {timestamp}" at the info level.
/// The message is processed by the logging system and sent to the appropriate
/// log targets.
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => ({
        $crate::core::diego_runtime::add_log_message($crate::logging::LogLevel::Info, format!($($arg)*));
    })
}
