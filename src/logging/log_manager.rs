use crate::logging::log_queue::LogQueue;
use crate::logging::{LogLevel, LogTarget};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogError {
    #[error("Logger name must not be empty. Please provide a valid, non-empty string to uniquely identify the logger."
    )]
    InvalidLoggerName,
    #[error("Logger with name `{0}` already exists")]
    LoggerExists(String),
    #[error("Logger name `{0}` does not exist")]
    LoggerNotFound(String),
    #[error("Given log target was of wrong type; required type is {0}, but it was {1}")]
    WrongTargetType(String, String),
}

pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
    pub log_time: Instant,
}

impl LogMessage {
    fn new(level: LogLevel, message: String) -> Self {
        Self { level, message, log_time: Instant::now() }
    }
}

pub(crate) struct LogManager {
    loggers: Arc<RwLock<HashMap<String, Box<dyn LogTarget>>>>,
    logger_count: u32,
    message_sender: Option<Sender<LogMessage>>,
    message_queue: Option<LogQueue>

}

impl LogManager {
    pub(crate) fn new() -> Self {
        Self {
            loggers: Arc::new(RwLock::new(HashMap::new())),
            logger_count: 0,
            message_sender: None,
            message_queue: None
        }
    }

    pub fn add_logger<N: AsRef<str>, T: LogTarget + 'static>(&mut self, name: N, log_target: T)
        -> Result<(), LogError>
    {
        // Check logger name (shouldn't be empty!)
        let logger_name = name.as_ref().to_owned();
        if logger_name.is_empty() {
            return Err(LogError::InvalidLoggerName);
        }


        // Obtain exclusive write access to the logger collection
        // and check if a logger with that name already exists.
        let mut loggers = self.loggers.write().unwrap();
        if loggers.contains_key(&logger_name) {
            return Err(LogError::LoggerExists(logger_name.to_owned()));
        }

        // Finally, add the logger to the collection
        loggers.insert(logger_name, Box::new(log_target));
        self.logger_count = loggers.len() as u32;
        drop(loggers);

        self.start_stop_processing();
        Ok(())
    }

    pub fn update_logger<N: AsRef<str>, T: LogTarget + 'static>(&mut self, name: N, log_target: T)
        -> Result<(), LogError>
    {
        // Check logger name (shouldn't be empty!)
        let logger_name = name.as_ref().to_owned();
        if logger_name.is_empty() {
            return Err(LogError::InvalidLoggerName);
        }

        // Obtain read-only access to the logger collection
        // since we don't modify it in this function
        let mut loggers = self.loggers.write().unwrap();
        match loggers.get_mut(&logger_name) {
            Some(existing_target) => {
                let target_name = log_target.target_name().to_owned();
                if existing_target.update(Box::new(log_target)) {
                    return Ok(());
                }
                Err(LogError::WrongTargetType(existing_target.target_name().to_owned(), target_name))
            },
            None => {
                Err(LogError::LoggerNotFound(logger_name.to_owned()))
            }
        }
    }

    pub fn remove_logger<N: AsRef<str>>(&mut self, name: N) -> Result<(), LogError> {
        let logger_name = name.as_ref().to_owned();
        if logger_name.is_empty() {
            return Err(LogError::InvalidLoggerName);
        }

        let mut loggers = self.loggers.write().unwrap();
        if loggers.remove(&logger_name).is_none() {
            return Err(LogError::LoggerNotFound(logger_name.to_owned()))
        }

        self.logger_count = loggers.len() as u32;
        drop(loggers);

        self.start_stop_processing();
        Ok(())
    }

    pub fn clear_all_loggers(&mut self) -> u32 {
        let logger_count;
        {
            let mut loggers = self.loggers.write().unwrap();
            logger_count = loggers.len() as u32;
            loggers.clear();
        }
        self.logger_count = 0;
        self.start_stop_processing();
        logger_count
    }

    pub fn get_logger_count(&self) -> u32 {
        self.logger_count
    }

    pub fn start_log_processing(&mut self) {
        if self.message_queue.is_none() {
            let create_and_run_result = LogQueue::create_and_run(self.loggers.clone());
            match create_and_run_result {
                Ok(res) => {
                    self.message_sender = Some(res.msg_sender);
                    self.message_queue = Some(res.log_queue);
                }
                Err(e) => {
                    eprintln!("Failed to create log queue: {}", e);
                }
            }
        }
    }

    pub fn stop_log_processing(&mut self) {
        if let Some(msq_queue) = self.message_queue.as_mut() {
            msq_queue.cancel();
            self.message_sender = None;
            self.message_queue = None;
        }
    }

    pub fn is_log_processing_active(&self) -> bool {
        self.message_queue.is_some()
    }

    fn start_stop_processing(&mut self) {
        if self.logger_count > 0 && self.message_queue.is_none() {
            self.start_log_processing();
        } else if self.logger_count == 0 && self.message_queue.is_some() {
            self.stop_log_processing();
        }
    }

    pub fn add_log_message(&mut self, level: LogLevel, message: String) {
        if let Some(sender) = self.message_sender.as_mut() {
            println!("Added log message: {} ({})", message, level);
            let log_message = LogMessage::new(level, message);

            if let Err(e) = sender.send(log_message) {
                eprintln!("Failed to send log message: {}", e);
            }
        }
    }
}
