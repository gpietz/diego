use crate::logging::{LogLevel, LogTarget};
use std::any::Any;
use crate::logging::log_manager::{LogError, LogMessage};

pub struct FileTarget {
    pub level: LogLevel,
    pub target_file: String,
}

impl LogTarget for FileTarget {
    fn update(&mut self, log_target: Box<dyn LogTarget>) -> bool {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn target_name(&self) -> &'static str {
        "FileTarget"
    }

    fn is_logging_enabled(&self, level: LogLevel) -> bool {
        level >= self.level
    }

    fn log(&self, log_message: &LogMessage) -> Result<(), LogError> {
        todo!()
    }
}
