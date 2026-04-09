// Демонстрация условной компиляции

use room_monitoring::RoomMetrics;

// Этот код будет компилироваться, только если активирована фича "foo"
#[cfg(feature = "foo")]
fn use_bar_feature() {
    println!("Фича 'foo' активирована, можно использовать функционал 'bar/baz'");
}

#[cfg(not(feature = "foo"))]
fn use_bar_feature() {
    println!("Фича 'foo' не активирована, этот код пропущен");
}

fn main() {
    println!("Демонстрация работы features");
    println!("===============================");

    // Генерируем тестовые метрики
    let metrics = RoomMetrics::random();

    println!("Сгенерированные метрики:");
    println!("  Температура: {:.1}°C", metrics.temperature);
    println!("  Влажность: {:.1}%", metrics.humidity);
    println!("  Давление: {:.1}hPa", metrics.pressure);
    println!(
        "  Дверь: {}",
        if metrics.door_open {
            "открыта"
        } else {
            "закрыта"
        }   
    );
    println!("  Влажность {:.1}", metrics.light_level);

    // Показываем, какие фичи активны
    #[cfg(feature = "random")]
    println!("\nФича 'random' активна");

    #[cfg(feature = "sqlite")]
    println!("Фича 'sqlite' активна");

    #[cfg(not(feature = "random"))]
    println!("\nФича 'random' отключена");

    #[cfg(not(feature = "sqlite"))]
    println!("Фича 'sqlite' отключена");

    // Демонстрация фичи sqlite
    #[cfg(feature = "sqlite")]
    {
        println!("\nSQL запрос:");
        println!("{}", metrics.to_sql());
    }

    // Демонстрация фичи sqlite
    #[cfg(feature = "logging")]
    {
        println!("\nФича logging включена:");
    }

    #[cfg(not(feature = "logging"))]
    {
        println!("\nФича logging отключена:");
    }

    // use_bar_feature();

}