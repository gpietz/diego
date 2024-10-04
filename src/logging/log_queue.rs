use crate::logging::log_manager::{LogError, LogManager, LogMessage};
use crate::logging::LogTarget;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

pub(crate) struct LogQueueCreateResult {
    pub msg_sender: Sender<LogMessage>,
    pub log_queue: LogQueue,
}

pub(crate) struct LogQueue {
    message_receiver: Arc<Mutex<Receiver<LogMessage>>>,
    loggers: Arc<RwLock<HashMap<String, Box<dyn LogTarget>>>>,
    cancel: Arc<RwLock<bool>>,
}

impl LogQueue {
    fn new(message_receiver: Receiver<LogMessage>,
               loggers: Arc<RwLock<HashMap<String, Box<dyn LogTarget>>>>) -> Self {
        Self {
            message_receiver: Arc::new(Mutex::new(message_receiver)),
            loggers,
            cancel: Arc::new(RwLock::new(false)),
        }
    }

    pub fn create_and_run(loggers: Arc<RwLock<HashMap<String, Box<dyn LogTarget>>>>)
        -> Result<LogQueueCreateResult, LogError> {
        let channel = mpsc::channel();
        let mut msg_queue = Self::new(channel.1, loggers);
        msg_queue.process_queue();
        let create_result = LogQueueCreateResult {
            msg_sender: channel.0,
            log_queue: msg_queue
        };
        Ok(create_result)
    }

    pub fn process_queue(&mut self) {
        let rx = self.message_receiver.clone();
        let loggers = Arc::clone(&self.loggers);
        let cancel = Arc::clone(&self.cancel);

        thread::spawn(move || {
            loop {
                while let Ok(log_message) = rx.lock().unwrap().recv() {
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

        println!("Exiting log queue");
    }

    pub fn cancel(&mut self) {
        *self.cancel.write().unwrap() = true;
    }
}
