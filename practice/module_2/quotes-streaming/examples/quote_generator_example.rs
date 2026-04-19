use std::{io, path::Path};
use std::time::Duration;
use std::sync::{Arc};
use quotes_streaming::quotes::{QuoteGenerator, QuoteError}; // Import the error type too

fn main() -> Result<(), QuoteError> {
    let quotes_generator_arc = Arc::new(QuoteGenerator::new());
    let quotes_generator_clone = quotes_generator_arc.clone();

    // Поток генератора цен
    std::thread::spawn(move || -> Result<(), QuoteError> {
        let loaded_count = quotes_generator_clone.load_tickers_from_file(Path::new("aux/tickers.txt"))?;
        println!("Загружено {} компаний: \n", loaded_count);

        loop {
            quotes_generator_clone.update_prices();
            std::thread::sleep(Duration::from_secs(2));
        }

        // Ok(())
    });

    println!("Проверка на одной компании: \t");
    println!("Введите компанию для проверки котировок: ");

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input)?;

    // 5 выводов по одной компании в разные tick'и
    for tick in 1..=5 {
        if let Some(quote) = quotes_generator_arc.generate_quote("AAPL")? {
            // Обновляем цены акций
            // quotes_generator_arc.update_prices();
            std::thread::sleep(Duration::from_secs(1));
            println!("  {:2}. Price: ${:8.2} | Volume: {:5} | Time: {}", 
                tick, quote.price, quote.volume, quote.timestamp);
        }
    }
    
    println!("\nНесколько компаний за один тик:");
    let tickers = ["AAPL", "MSFT", "GOOGL", "TSLA", "UNKNOWN"];
    
    for ticker in &tickers {
        if let Some(quote) = quotes_generator_arc.generate_quote(ticker)? {
            println!("  {:6} | ${:8.2} | Vol: {:5}", 
                quote.ticker, quote.price, quote.volume);
        }
    }
    
    println!("\nПроверка текущих ценТекущие цены:");
    for ticker in &["AAPL", "MSFT", "TSLA"] {
        if let Some(price) = quotes_generator_arc.get_current_price(ticker)? {
            println!("  {:6}: ${:.2}", ticker, price);
        }
    }
    
    Ok(())
}