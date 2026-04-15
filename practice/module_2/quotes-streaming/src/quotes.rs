use std::time::{SystemTime, UNIX_EPOCH};
use std::net::SocketAddr;
use rand::Rng;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::RwLock;
use std::time::Duration;
use crossbeam_channel as channel;

type Company = String;

#[derive(Debug, Clone)]
pub struct StockQuote {
    pub ticker: Company,
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
    // Примечание: вследствие Arc все методы надо поменять на &self для внутренней мутабельности,
    // Arc реализует DeRef, поэтому вызов функций происходит на &self
    stock_prices_rw: RwLock<HashMap<Company, f64>>,
    udp_streamer_channels_rw: RwLock<HashMap<SocketAddr, UdpStreamSubscriber>>,
}

// Отдельные UDP-потоки принимают данные с подпиской, предварительно зарегистрировавшись в QuoteGenerator'е
pub struct UdpStreamSubscriber {
    pub to_streaming_thread_channel: channel::Sender<StockQuote>,
    pub tickers: Vec<String>,
}


impl QuoteGenerator {
    pub fn new() -> Self {
        Self {
            // stock_prices: HashMap::new(),
            stock_prices_rw: RwLock::new(HashMap::new()),
            udp_streamer_channels_rw: RwLock::new(HashMap::new())
        }
    }

    // Функция загрузки тикеров из файла - устанавливает начальное значение цены
    pub fn load_tickers_from_file(&self, path: &Path) -> std::io::Result<usize> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut rand_gen = rand::rng();
        let mut num_of_companies = 0;

        // Ставим lock на время наполнения
        let mut stock_prices = self.stock_prices_rw.write().unwrap();

        // Особо не проверяю формат и наличие ячейки в HashMap
        for line in reader.lines() {
            let line = line?;
            let cur_ticker = line.trim();
            
            let initial_price = rand_gen.random_range(100.0..=500.0);
            stock_prices.insert(cur_ticker.to_string(), initial_price);
            num_of_companies += 1;
        }
        
        Ok(num_of_companies)
    }
    
    // Добавить вручную компании
    pub fn add_ticker(&self, ticker: &str, initial_price: f64) {
        // Lock на запись
        let mut stock_prices = self.stock_prices_rw.write().unwrap();
        stock_prices.insert(ticker.to_string(), initial_price);
    }

    
    // Изменить цены
    pub fn update_prices(&self) {
        // Lock на запись
        let mut stock_prices = self.stock_prices_rw.write().unwrap();

        for company_stock_price in stock_prices.values_mut() {            
                let mut rand_gen = rand::rng();
                let change_percent = rand_gen.random_range(-0.15..=0.15);
                *company_stock_price = *company_stock_price * (1.0 + change_percent);
                
                // Можно задать границы - удобно
                *company_stock_price = company_stock_price.max(0.01).min(10000.0);
        } 
    }

    /// Получить текущее значение цены
    pub fn get_current_price(&self, ticker: &str) -> Option<f64> {
        // Lock на чтение
        let stock_prices = self.stock_prices_rw.read().unwrap();
        stock_prices.get(ticker).copied()
    }
    
    // Сгенерировать котировку
    pub fn generate_quote(&self, ticker: &str) -> Option<StockQuote> {
        // Генерируем объём
        let volume = match ticker {
            // Популярные акции имеют больший объём
            "AAPL" | "MSFT" | "TSLA" => 1000 + (rand::random::<f64>() * 5000.0) as u32,
            // Обычные акции - средний объём
            _ => 100 + (rand::random::<f64>() * 1000.0) as u32,
        };
        
        // Lock на чтение - лок не нужен, делаем в get_current_price
        // let stock_prices = self.stock_prices_rwlocked.read().unwrap();
        Some(StockQuote {
            ticker: ticker.to_string(),
            price: Self::get_current_price(&self, ticker)?,
            volume,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
        })
    }


    /// Регистрация "подписчиков" (UDP-стримеров) с указанными тикерами
    pub fn register_udp_streaming(&self, client_addr: SocketAddr, tickers_subscribe: Vec<String>) -> Result<channel::Receiver<StockQuote>, String> {
        // Лочим зарегистрированных на запись
        let mut streamers = self.udp_streamer_channels_rw.write().unwrap();
 
        // Создаём канал для UDP-потока-стримера
        let (send_to_client_channel, receive_from_gen_channel) = channel::unbounded::<StockQuote>();

        // Проверяем, что стриминг уже начался на этого клиента
        Self::is_streaming_already_started(&self, &client_addr, &streamers)?;

        // Сохраняем канал для отправки клиенту 
        streamers.insert(client_addr, UdpStreamSubscriber {
            to_streaming_thread_channel: send_to_client_channel,
            tickers: tickers_subscribe.clone(),
        });
        
        println!("👤 UDP-стрим-поток для {} зарегистрировался с тикерами {:?}", client_addr, tickers_subscribe);
        
        Ok(receive_from_gen_channel)
    }
    
    /// Снять с регистрации
    pub fn deregister_udp_streaming(&self, client_addr: SocketAddr) -> Option<SocketAddr> {
        // Лочим зарегистрированных на запись
        let mut streamers = self.udp_streamer_channels_rw.write().unwrap();
 
        // Проверяем, что не было ничего 
        if let Some(_) = streamers.remove(&client_addr) {
            println!("🗙 UDP-стрим снят с подписок: {} ", client_addr);
            return Some(client_addr);
        }
        
        println!("Попытка разрегистрации незарегистрированного UDP-стрима: {}", client_addr);
        
        None
    }

    // Вручную послать котировку клиенту
    pub fn send_quote_to_client(&self, client_addr: &SocketAddr, quote: StockQuote) -> Result<(), String> {
        let channels = self.udp_streamer_channels_rw.read().unwrap();
        
        if let Some(client_channel) = channels.get(client_addr) {
            // Проверка, подписан ли на текущий тикер
            if client_channel.tickers.contains(&quote.ticker) {
                client_channel.to_streaming_thread_channel.send(quote)
                    .map_err(|e| format!("Failed to send quote: {}", e))?;
                Ok(())
            } else {
                Err("UDP-стример не подписан на текущий тикер".to_string())
            }
        } else {
            Err("Клиент не найден".to_string())
        }
    }
    
    // Послать каждому подписчику котировки тикеров, на которые он подписан
    pub fn broadcast_quote(&self, quote: &StockQuote) {
        let streamers = self.udp_streamer_channels_rw.read().unwrap();
        
        for (_, client_channel) in streamers.iter() {
            if client_channel.tickers.contains(&quote.ticker) {
                let _ = client_channel.to_streaming_thread_channel.send(quote.clone());
            }
        }
    }
    
    // Однократно послать котировки, на которые подписаны (основной цикл при создании потока)
    pub fn broadcast_quotes_to_subscribers(&self) {
        let stock_prices = self.stock_prices_rw.read().unwrap();

        for (ticker, _) in stock_prices.iter() {

            // TODO: обработать ошибку
            let quote = Self::generate_quote(&self, ticker).unwrap();
            self.broadcast_quote(&quote);
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    // Проверка, начался ли уже стриминг на указанный адрес:порт
    // TODO: надо делать атомарно с локом, пока чуть костыльно сделал
    fn is_streaming_already_started(&self, client_addr: &SocketAddr, 
                                    streamers: &std::sync::RwLockWriteGuard<'_, HashMap<SocketAddr, UdpStreamSubscriber>>)
                                     -> Result<(), String> 
    {
        // Лок на чтение
        // let streamers = self.udp_streamer_channels_rw.read().unwrap();
        
        if streamers.contains_key(client_addr) {
            return Err(format!("Стриминг уже начался на указанный адрес:порт: {}", client_addr));
        }

        Ok(())
    }
} 