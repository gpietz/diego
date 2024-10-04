
/// Logs a message with `LogLevel::Trace`.
///
/// This macro allows you to log debug-level messages, which are typically used
/// for development and troubleshooting. It accepts any number of arguments and
/// formats them into a string using the `format!` macro.
///
/// # Example
///
/// ```no_run
/// log_trace!("Debugging value: {}", value);
/// ```
///
/// This will log the message "Trace value: {value}" at the debug level.
/// The message is processed by the logging system and sent to the appropriate
/// log targets.
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => ({
        $crate::core::diego_runtime::add_log_message($crate::logging::LogLevel::Trace, format!($($arg)*));
    })
}

/// Logs a message with `LogLevel::Debug`.
///
/// This macro allows you to log debug-level messages, which are typically used
/// for development and troubleshooting. It accepts any number of arguments and
/// formats them into a string using the `format!` macro.
///
/// # Example
///
/// ```no_run
/// log_debug!("Debugging value: {}", value);
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
/// log_info!("Application started at: {}", timestamp);
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

/// Logs a warning-level message using the `add_log_message` function
/// within the Diego runtime.
///
/// This macro accepts any number of arguments that can be formatted into a string,
/// similar to the `format!` macro.
///
/// # Example
///
/// ```no_run
/// log_warn!("This is a warning message: {}", some_variable);
/// ```
///
/// The message will be passed to the `add_log_message` function in the `diego_runtime` module,
/// with a log level of `Warn`.
///
/// # Parameters
/// * `$($arg:tt)*`: A variadic argument list, similar to the standard `println!` macro, which
///   allows formatting of the message string.
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => ({
        $crate::core::diego_runtime::add_log_message($crate::logging::LogLevel::Warn, format!($($arg)*));
    })
}

/// Logs an error-level message using the `add_log_message` function within the Diego runtime.
///
/// This macro accepts any number of arguments that can be formatted into a string,
/// similar to the `format!` macro.
///
/// # Example
///
/// ```no_run
/// log_error!("This is an error message: {}", some_variable);
/// ```
///
/// The message will be passed to the `add_log_message` function in the `diego_runtime` module,
/// with a log level of `Error`.
///
/// # Parameters
/// * `$($arg:tt)*`: A variadic argument list, similar to the standard `println!` macro, which
/// allows formatting of the message string.
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => ({
        $crate::core::diego_runtime::add_log_message($crate::logging::LogLevel::Error, format!($($arg)*));
    })
}

/// Logs a fatal-level message using the `add_log_message` function within the Diego runtime.
///
/// This macro accepts any number of arguments that can be formatted into a string,
/// similar to the `format!` macro. Fatal logs indicate critical issues that may cause the
/// program to terminate.
///
/// # Example
///
/// ```no_run
/// log_fatal!("This is a fatal error: {}", some_variable);
/// ```
///
/// The message will be passed to the `add_log_message` function in the `diego_runtime` module,
/// with a log level of `Fatal`.
///
/// # Parameters
/// * `$($arg:tt)*`: A variadic argument list, similar to the standard `println!` macro, which
///   allows formatting of the message string.
#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => ({
        $crate::core::diego_runtime::add_log_message($crate::logging::LogLevel::Fatal, format!($($arg)*));
    })
}
