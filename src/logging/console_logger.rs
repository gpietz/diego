use std::any::Any;
use crate::logging::{log_formatter, LogLevel, LogTarget};
use crate::logging::log_formatter::format_message;
use crate::logging::log_manager::{LogError, LogMessage};

#[derive(Debug)]
pub struct ConsoleTarget {
    pub level: LogLevel,
    pub format: String,
}

impl Default for ConsoleTarget {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: log_formatter::DEFAULT_FORMAT.to_string(),
        }
    }
}

impl LogTarget for ConsoleTarget {
    fn update(&mut self, log_target: Box<dyn LogTarget>) -> bool {
        if let Some(target) = log_target.as_any().downcast_ref::<Self>() {
            self.level = target.level;
            return true;
        }
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn target_name(&self) -> &'static str {
        "ConsoleTarget"
    }

    fn is_logging_enabled(&self, level: LogLevel) -> bool {
        level >= self.level
    }

    fn log(&self, log_message: &LogMessage) -> Result<(), LogError> {
        if self.is_logging_enabled(log_message.level) {
            println!("{}", format_message(&self.format, log_message));
        }
        Ok(())
    }
}
