use std::any::Any;
use crate::logging::{LogLevel, LogTarget};
use crate::logging::log_manager::{LogError, LogMessage};

#[derive(Debug, Default)]
pub struct ConsoleTarget {
    pub level: LogLevel,
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
            println!("{}", log_message.message);
        }
        Ok(())
    }
}
