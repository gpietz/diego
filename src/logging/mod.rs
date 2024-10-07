use std::any::Any;
use std::fmt::{Debug, Display, Formatter, Result};
use crate::logging::log_manager::{LogError, LogMessage};
use std::result::Result as StdResult;

pub mod log_manager;
pub mod file_logger;
pub mod console_logger;
pub mod network_logger;
pub mod log_queue;

mod log_formatter;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
    }
}

pub trait LogTarget: Any + Send + Sync {
    fn update(&mut self, log_target: Box<dyn LogTarget>) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn target_name(&self) -> &'static str;
    fn is_logging_enabled(&self, level: LogLevel) -> bool;
    fn log(&self, log_message: &LogMessage) -> StdResult<(), LogError>;
}
