use std::{io, path::Path};
use quotes_streaming::quotes::{QuoteGenerator, StockQuote};
// use std::io::stdin;

fn main() -> std::io::Result<()> {
    let mut quote_generator = QuoteGenerator::new();
    
    let loaded_count = quote_generator.load_tickers_from_file(Path::new("aux/tickers.txt"))?;
    println!("Загружено {} компаний: \n", loaded_count);
    
    println!("Проверка на одной компании: \t");
    println!("Введите компанию для проверки котировок: ");

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    
    for tick in 1..=5 {
        if let Some(quote) = quote_generator.generate_quote("AAPL") {
            println!("  {:2}. Price: ${:8.2} | Volume: {:5} | Time: {}", 
                tick, quote.price, quote.volume, quote.timestamp);
        }
    }
    
    println!("\nНесколько компаний за один тик:");
    let tickers = ["AAPL", "MSFT", "GOOGL", "TSLA", "UNKNOWN"];
    
    for ticker in &tickers {
        if let Some(quote) = quote_generator.generate_quote(ticker) {
            println!("  {:6} | ${:8.2} | Vol: {:5}", 
                quote.ticker, quote.price, quote.volume);
        }
    }
    
    println!("\nПроверка текущих ценТекущие цены:");
    for ticker in &["AAPL", "MSFT", "TSLA"] {
        if let Some(price) = quote_generator.get_current_price(ticker) {
            println!("  {:6}: ${:.2}", ticker, price);
        }
    }
    
    Ok(())
}