use std::any::Any;
use std::sync::Mutex;
use std::io::{Write, stdout};

pub trait Logger {
    fn log(&self, message: &str);

    // важно: возвращаем &dyn Any, а не Self
    // и не делаем метод generic — тогда он будет object safe
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub struct ConsoleLogger;
pub struct MemoryLogger {
    // Так не сработает
    // log_messages: Vec<String>
    log_messages: Mutex<Vec<String>>
}

impl MemoryLogger {
    pub fn new() -> Self {
        Self {
            log_messages: Mutex::new(Vec::new()),
        }
    }

    pub fn get_entries(&self) -> Vec<String> {
        self.log_messages.lock().unwrap().clone()
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        stdout().write_all(message.as_bytes());
        stdout().flush();

        // Или println!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Logger for MemoryLogger {
    fn log(&self, message: &str) {
        // Так сработает - через мьютекс
        self.log_messages.lock().unwrap().push(message.to_string());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}