use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};


type Company = String;

#[derive(Debug, Clone)]
pub struct StockQuote {
    pub ticker: String,
    pub price: f64,
    pub volume: u32,
    pub timestamp: u64,
}

// Методы для сериализации/десериализации
impl StockQuote {
    pub fn to_string(&self) -> String {
        format!("{}|{}|{}|{}", self.ticker, self.price, self.volume, self.timestamp)
    }
    
    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() == 4 {
            Some(StockQuote {
                ticker: parts[0].to_string(),
                price: parts[1].parse().ok()?,
                volume: parts[2].parse().ok()?,
                timestamp: parts[3].parse().ok()?,
            })
        } else {
            None
        }
    }
    
    // Или бинарная сериализация
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.ticker.as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.price.to_string().as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.volume.to_string().as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.timestamp.to_string().as_bytes());
        bytes
    }
}

pub struct QuoteGenerator {
    // Храним цены в мап'е: название -> текущая цена
    current_prices: HashMap<Company, f64>,
}

impl QuoteGenerator {
    pub fn new() -> Self {
        Self {
            current_prices: HashMap::new(),
        }
    }

    // Функция загрузки тикеров из файла - устанавливает начальное значение цены
    pub fn load_tickers_from_file(&mut self, path: &Path) -> std::io::Result<usize> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut rand_gen = rand::rng();
        let mut num_of_companies = 0;

        // Особо не проверяю формат и наличие ячейки в HashMap
        for line in reader.lines() {
            let line = line?;
            let cur_ticker = line.trim();
            
            let initial_price = rand_gen.random_range(100.0..=500.0);
            self.current_prices.insert(cur_ticker.to_string(), initial_price);
            num_of_companies += 1;
        }
        
        Ok(num_of_companies)
    }
    
    // Добавить отдельно компании
    pub fn add_ticker(&mut self, ticker: &str, initial_price: f64) {
        self.current_prices.insert(ticker.to_string(), initial_price);
    }

    // Сгенерировать котировку
    pub fn generate_quote(&mut self, ticker: &str) -> Option<StockQuote> {
        // Берём текущую цену или вставляем так же, как при загрузке
        let current_price = self.current_prices.entry(ticker.to_string())
            .or_insert_with(|| {
                let mut rng = rand::rng();
                rng.random_range(100.0..=500.0)
            });
        
        let mut rng = rand::rng();
        let change_percent = rng.random_range(-0.15..=0.15);
        *current_price = *current_price * (1.0 + change_percent);
        
        // Можно задать границы - удобно
        *current_price = current_price.max(0.01).min(10000.0);
        
        let volume = match ticker {
            // Популярные акции имеют больший объём
            "AAPL" | "MSFT" | "TSLA" => 1000 + (rand::random::<f64>() * 5000.0) as u32,
            // Обычные акции - средний объём
            _ => 100 + (rand::random::<f64>() * 1000.0) as u32,
        };
        
        Some(StockQuote {
            ticker: ticker.to_string(),
            price: *current_price,
            volume,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
        })
    }

    /// Получить текущее значение цены
    pub fn get_current_price(&self, ticker: &str) -> Option<f64> {
        self.current_prices.get(ticker).copied()
    }
} 