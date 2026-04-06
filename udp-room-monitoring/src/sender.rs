use crate::metrics::RoomMetrics;
use bincode;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

pub struct MetricsSender {
    // Тоже только сокет
    socket: UdpSocket,
}

impl MetricsSender {
    pub fn new(bind_addr: &str) -> Result<Self, std::io::Error> {
        // Биндим к адресу
        let socket = UdpSocket::bind(bind_addr)?;
        Ok(Self { socket })
    }

    // Метод отправки сообщений в сокет
    pub fn send_to(
        &self,
        metrics: &RoomMetrics,
        target_addr: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Отправляем сериализовнную версию метрик
        let encoded = bincode::serialize(metrics)?;
        self.socket.send_to(&encoded, target_addr)?;
        Ok(())
    }

    // Метод для запуска цикла постоянной отправки метрик
    pub fn start_broadcasting(
        self,
        target_addr: String,
        interval_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Имитатор датчиков запущен. Отправка на {} каждые {}ms",
            target_addr, interval_ms
        );

        loop {
            let metrics = RoomMetrics::random();

            match self.send_to(&metrics, &target_addr) {
                Ok(()) => {
                    println!(
                        "[{}] Отправлено: {:.1}C, {:.1}% влажности, давление: {:.1}hPa, дверь: {}, освещённость {:.1}",
                        metrics.formatted_time(),
                        metrics.temperature,
                        metrics.humidity,
                        metrics.pressure,
                        if metrics.door_open {
                            "открыта"
                        } else {
                            "закрыта"
                        },
                        metrics.light_level,
                    );
                }
                Err(e) => {
                    eprintln!("Ошибка отправки: {}", e);
                }
            }

            thread::sleep(Duration::from_millis(interval_ms));
        }
    }
}