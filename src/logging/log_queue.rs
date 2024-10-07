use crate::logging::log_manager::{LogError, LogMessage};
use crate::logging::LogTarget;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub(crate) struct LogQueue {
    message_queue: Arc<RwLock<VecDeque<LogMessage>>>,
    loggers: Arc<RwLock<HashMap<String, Box<dyn LogTarget>>>>,
    cancel: Arc<RwLock<bool>>,
}

impl LogQueue {
    fn new(loggers: Arc<RwLock<HashMap<String, Box<dyn LogTarget>>>>) -> Self {
        Self {
            message_queue: Arc::new(RwLock::new(VecDeque::new())),
            loggers,
            cancel: Arc::new(RwLock::new(false)),
        }
    }

    pub fn create_and_run(loggers: Arc<RwLock<HashMap<String, Box<dyn LogTarget>>>>)
        -> Result<Self, LogError> {
        let mut msg_queue = Self::new(loggers);
        msg_queue.process_queue();
        Ok(msg_queue)
    }

    fn process_queue(&mut self) {
        let loggers = Arc::clone(&self.loggers);
        let cancel = Arc::clone(&self.cancel);
        let msg_queue = Arc::clone(&self.message_queue);

        thread::spawn(move || {
            loop {
                let mut queue = msg_queue.write().unwrap();
                while let Some(log_message) = queue.pop_front() {
                    let loggers = loggers.read().unwrap();
                    for logger in loggers.values() {
                        if let Err(e) = logger.log(&log_message) {
                            eprintln!("Error logging message: {}", e);
                            // TODO: Add extended error logging (file etc.)
                        }
                    }
                }                 
                
                if *cancel.read().unwrap() {
                    break;
                }

                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn cancel(&mut self) {
        *self.cancel.write().unwrap() = true;
    }

    pub fn push_message(&self, log_message: LogMessage) {
        let mut queue = self.message_queue.write().unwrap();
        queue.push_back(log_message);
    }
}
