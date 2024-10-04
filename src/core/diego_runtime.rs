use crate::logging::log_manager::{LogError, LogManager};
use crate::logging::{LogLevel, LogTarget};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::core::diego_runtime_internal::DiegoRuntimeInternal;

lazy_static! {
    static ref DIEGO_RUNTIME: Arc<Mutex<DiegoRuntimeInternal>> = Arc::new(Mutex::new(DiegoRuntimeInternal::new()));
    static ref LOG_MANAGER: Arc<Mutex<LogManager>> = Arc::new(Mutex::new(LogManager::new()));
}

/// Enables or disables runtime checking for OpenGL errors.
///
/// # Arguments
///
/// * `check_enabled` - A boolean value indicating whether to enable (`true`) or disable (`false`)
///   the OpenGL error checking during runtime.
///
/// This function locks the DIEGO_RUNTIME instance and sets the `check_opengl_errors` flag
/// accordingly, which determines if OpenGL error checking is performed during execution.
///
/// # Example
/// ```no_run
/// set_check_opengl_errors(true); // Enables OpenGL error checking.
/// set_check_opengl_errors(false); // Disables OpenGL error checking.
/// ```
pub fn set_check_opengl_errors(check_enabled: bool) {
    let mut runtime = DIEGO_RUNTIME.lock().unwrap();
    runtime.check_opengl_errors = check_enabled;
}

pub fn check_opengl_errors() -> bool {
    let runtime = DIEGO_RUNTIME.lock().unwrap();
    runtime.check_opengl_errors
}

pub fn add_logger<N: AsRef<str>, T: LogTarget + 'static>(name: N, log_target: T)  
    -> Result<(), LogError> {
    let mut log_manager = LOG_MANAGER.lock().unwrap();
    log_manager.add_logger(name, log_target)
}

pub fn try_add_logger<N: AsRef<str>, T: LogTarget + 'static>(name: N, log_target: T) -> bool {
    add_logger(name, log_target).is_ok()
}

pub fn update_logger<N: AsRef<str>, T: LogTarget + 'static>(name: N, log_target: T) 
    -> Result<(), LogError> {
    let mut log_manager = LOG_MANAGER.lock().unwrap();
    log_manager.update_logger(name, log_target)
}

pub fn try_update_logger<N: AsRef<str>, T: LogTarget + 'static>(name: N, log_target: T) -> bool {
    add_logger(name, log_target).is_ok()
}

pub fn remove_logger<N: AsRef<str>>(name: N) -> Result<(), LogError> {
    let mut log_manager = LOG_MANAGER.lock().unwrap();
    log_manager.remove_logger(name)
}

pub fn try_remove_logger<N: AsRef<str>>(name: N) -> bool {
    remove_logger(name).is_ok()
}

pub fn get_logger_count() -> u32 {
    let log_manager = LOG_MANAGER.lock().unwrap();
    log_manager.get_logger_count()
}

pub fn clear_all_loggers() -> u32 {
    let mut log_manager = LOG_MANAGER.lock().unwrap();
    log_manager.clear_all_loggers()
}

pub fn add_log_message<T: AsRef<str>>(log_level: LogLevel, log_message: T) {
    let message = log_message.as_ref().to_owned();
    let mut log_manager = LOG_MANAGER.lock().unwrap();
    log_manager.add_log_message(log_level, message);
}
