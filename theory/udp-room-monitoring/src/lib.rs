pub mod metrics;
pub mod receiver;
pub mod sender;
pub mod logger;

pub use metrics::RoomMetrics;
pub use receiver::MetricsReceiver;
pub use sender::MetricsSender; 

// Условно компилируем модуль логирования
#[cfg(feature = "logging")]
pub mod logging;

#[cfg(not(feature = "logging"))]
mod logging;

// Реэкспортируем макросы логирования
#[cfg(feature = "logging")]
pub use logging::{debug, error, info, trace, warn, init_logger};