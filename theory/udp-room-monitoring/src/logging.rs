use std::io::{Write, stdout};

#[cfg(feature = "logging")]
pub use log::{debug, error, info, trace, warn};

// Макросы-заглушки, когда фича logging отключена
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

// Функция инициализации логирования
#[cfg(feature = "logging")]
pub fn init_logger() {
    env_logger::init();
    info!("Логирование инициализировано");
}

#[cfg(not(feature = "logging"))]
pub fn init_logger() {
    // Ничего не делаем, когда логирование отключено
}

use std::any::Any;
use std::sync::Mutex;

trait Logger {
    fn log(&self, message: &str);

    // важно: возвращаем &dyn Any, а не Self
    // и не делаем метод generic — тогда он будет object safe
    fn as_any(&self) -> &dyn Any;
}

struct ConsoleLogger;
struct MemoryLogger {
    // Так не сработает
    // log_messages: Vec<String>
    log_messages: Mutex<Vec<String>>
}

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        stdout().write_all(message.as_bytes());
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