use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMetrics {
    pub timestamp: u64,   // Unix timestamp в секундах
    pub temperature: f32, // °C
    pub humidity: f32,    // %
    pub pressure: f32,    // hPa
    pub door_open: bool,
    // Добавляю в задании
    pub light_level: f32,
}

impl RoomMetrics {
    pub fn new(temperature: f32, humidity: f32, pressure: f32, door_open: bool, light_level: f32) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            temperature,
            humidity,
            pressure,
            door_open,
            light_level,
        }
    }

    // Метод для имитации метрик
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        Self::new(
            rng.random_range(18.0..25.0),
            rng.random_range(30.0..60.0),
            rng.random_range(980.0..1020.0),
            rng.random_bool(0.1), // 10% chance door is open
            rng.random_range(0.5..0.8), // Освещённость
        )
    }

    // Метод для форматированного отображения времени
    pub fn formatted_time(&self) -> String {
        format!("{}s", self.timestamp)
    }
}