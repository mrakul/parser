// src/bins/monitor/main.rs

use room_monitoring::MetricsReceiver;
use room_monitoring::receiver::Receiver;
use room_monitoring::receiver::MockReceiver;
use room_monitoring::logger::{*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_addr = "127.0.0.1:8080";

    let console_logger = Box::new(ConsoleLogger);
    let memory_logger = Box::new(MemoryLogger::new());

    // Вектор трейтовых объектов
    let loggers: Vec<Box<dyn Logger>> = vec![console_logger.clone(), memory_logger];

    console_logger.log(" Запуск системы мониторинга банковского хранилища");
    console_logger.log(&format!("Прослушивание адреса: {}", bind_addr));
    console_logger.log("──────────────────────────────────────────────────");

    // println!(" Запуск системы мониторинга банковского хранилища");
    // println!("Прослушивание адреса: {}", bind_addr);
    // println!("──────────────────────────────────────────────────");

    // Здесь меняем для динамической диспетчеризации (проверка переменной окружения)
    //  Обычно:
    //     export USE_MOCK=1
    // Или при запуске:
    //      USE_MOCK=1 cargo run
    let receiver: Box<dyn Receiver> = if std::env::var("USE_MOCK").is_ok() {
        Box::new(MockReceiver)
    } else {
        Box::new(MetricsReceiver::new(bind_addr)?)
    }; 
    // let receiver = MetricsReceiver::new(bind_addr)?;

    let (receiver_handle, metrics_rx) = receiver.start_with_channel();

    console_logger.log("Система мониторинга запущена. Ожидание данных...");
    console_logger.log("Нажмите Ctrl+C для остановки");

    // println!("Система мониторинга запущена. Ожидание данных...");
    // println!("Нажмите Ctrl+C для остановки");

    let mut total_received = 0;

    // Основной цикл обработки данных
    loop {
        match metrics_rx.recv() {
            Ok((metrics, _src_addr)) => {
                total_received += 1;

                // Определяем статус тревоги
                let alert_status = if metrics.door_open {
                    "🚨 ТРЕВОГА: ДВЕРЬ ОТКРЫТА!"
                } else if metrics.temperature > 30.0 {
                    "⚠️  ВНИМАНИЕ: Высокая температура"
                } else if metrics.humidity > 70.0 {
                    "⚠️  ВНИМАНИЕ: Высокая влажность"
                } else {
                    "✅ Норма"
                };

                // (!) Без логгеров
                // println!(
                //     "[#{:03}] {} | Темп: {:.1}°C | Влажн: {:.1}% | Давл: {:.1}hPa | Дверь: {} | {} | освещённость: {:.1}",
                //     total_received,
                //     metrics.formatted_time(),
                //     metrics.temperature,
                //     metrics.humidity,
                //     metrics.pressure,
                //     if metrics.door_open {
                //         "ОТКРЫТА"
                //     } else {
                //         "закрыта"
                //     },
                //     alert_status,
                //     metrics.light_level
                // );
                    for logger in &loggers {
                        logger.log(&format!(
                        "[#{:03}] {} | Темп: {:.1}°C | Влажн: {:.1}% | Давл: {:.1}hPa | Дверь: {} | {} | CO2 уровень: {:.2}| ",
                        total_received,
                        metrics.formatted_time(),
                        metrics.temperature,
                        metrics.humidity,
                        metrics.pressure,
                        if metrics.door_open {
                            "ОТКРЫТА"
                        } else {
                            "закрыта"
                        },
                        alert_status,
                        metrics.light_level,
                        ));
                    }
            }
            Err(_) => {
                println!("🔌 Канал закрыт. Завершение работы.");
                break;
            }
        }
    }

    // Пытаемся дождаться завершения потока
    let _ = receiver_handle.join();
    for logger in &loggers {
        if let Some(mem) = logger.as_any().downcast_ref::<MemoryLogger>() {
            println!("Содержимое MemoryLogger:");
            for entry in mem.get_entries() {
                println!("  {entry}");
            }
        }
    }
    println!("Итог: получено {} пакетов данных", total_received);
    Ok(())
} 