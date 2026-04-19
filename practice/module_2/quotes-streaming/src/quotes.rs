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
use std::sync::PoisonError;

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

#[derive(Debug)]
// Несколько возможных типов ошибок
pub enum QuoteError {
    IoError(std::io::Error),
    LockPoisoned(String),
    ClientAlreadyRegistered(SocketAddr),
    SendQuoteError(String),
}

// Для вывода QuoteError
impl std::fmt::Display for QuoteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteError::IoError(e) => write!(f, "IO Error: {}", e),
            QuoteError::LockPoisoned(msg) => write!(f, "Lock Poisoned: {}", msg),
            QuoteError::ClientAlreadyRegistered(registered_addr) => write!(f, "Клиент уже зарегистрирован для адреса: {}", registered_addr),
            QuoteError::SendQuoteError(reason) => write!(f, "Ошибка посылки котировки: {}", reason),
        }
    }
}

// impl std::error::Error for QuoteError {}

impl From<std::io::Error> for QuoteError {
    fn from(err: std::io::Error) -> Self {
        QuoteError::IoError(err)
    }
}

// Темплейтная from для преобразования ошибки
impl<T> From<PoisonError<T>> for QuoteError {
    fn from(_: PoisonError<T>) -> Self {
        QuoteError::LockPoisoned("Внутренняя ошибка RwLock - RwLock в состоянии 'poisoned'".to_string())
    }
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
    pub fn load_tickers_from_file(&self, path: &Path) -> Result<usize, QuoteError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut rand_gen = rand::rng();
        let mut num_of_companies = 0;

        // Ставим lock на время наполнения
        let mut stock_prices = self.stock_prices_rw.write()?;

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
    pub fn add_ticker(&self, ticker: &str, initial_price: f64) -> Result<(), QuoteError> {
        // Lock на запись
        let mut stock_prices = self.stock_prices_rw.write()?;
        
        stock_prices.insert(ticker.to_string(), initial_price);

        Ok(())
    }

    
    // Изменить цены
    pub fn update_prices(&self) -> Result<(), QuoteError> {
        // Lock на запись
        let mut stock_prices = self.stock_prices_rw.write()?;

        for company_stock_price in stock_prices.values_mut() {            
                let mut rand_gen = rand::rng();
                let change_percent = rand_gen.random_range(-0.15..=0.15);
                *company_stock_price = *company_stock_price * (1.0 + change_percent);
                
                // Можно задать границы - удобно
                *company_stock_price = company_stock_price.max(0.01).min(10000.0);
        } 

        Ok(())
    }

    /// Получить текущее значение цены
    pub fn get_current_price(&self, ticker: &str) -> Result<Option<f64>, QuoteError> {
        // Lock на чтение
        let stock_prices = self.stock_prices_rw.read()?;
        Ok(stock_prices.get(ticker).copied())
    }
    
    // Сгенерировать котировку
    pub fn generate_quote(&self, ticker: &str) -> Result<Option<StockQuote>, QuoteError> {
        // Генерируем объём
        let volume = match ticker {
            // Популярные акции имеют больший объём
            "AAPL" | "MSFT" | "TSLA" => 1000 + (rand::random::<f64>() * 5000.0) as u32,
            // Обычные акции - средний объём
            _ => 100 + (rand::random::<f64>() * 1000.0) as u32,
        };
        
        // Lock на чтение - лок не нужен, делаем в get_current_price
        let quote_option = Self::get_current_price(self, ticker)?
            .map(|got_price_value| StockQuote {
                ticker: ticker.to_string(),
                price: got_price_value,
                volume,
                // Оставил .unwrap() намеренно, поскольку крайне меловероятно, и здесь это подходящий подход, насколько понял
                // Но суть ошибки понял, если текущая дата установлена меньше UNIX_EPOCH (можно поменять на expect())
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
            });

        Ok(quote_option)
    }


    /// Регистрация "подписчиков" (UDP-стримеров) с указанными тикерами
    pub fn register_udp_streaming(&self, client_addr: SocketAddr, tickers_subscribe: Vec<String>) -> Result<channel::Receiver<StockQuote>, QuoteError> {
        // Лочим зарегистрированных на запись, пробрасываем ошибку
        let mut streamers = self.udp_streamer_channels_rw.write()?;
 
        // Создаём канал для UDP-потока-стримера
        let (send_to_client_channel, receive_from_gen_channel) = channel::unbounded::<StockQuote>();

        // Проверяем, что стриминг уже начался на этого клиента
        Self::is_streaming_already_started(&self, &client_addr, &streamers)?;

        // Сохраняем канал для отправки клиенту 
        streamers.insert(client_addr, UdpStreamSubscriber {
            to_streaming_thread_channel: send_to_client_channel,
            tickers: tickers_subscribe.clone(),
        });
        
        log::info!("👤 UDP-стрим-поток для {} зарегистрировался с тикерами {:?}", client_addr, tickers_subscribe);
        
        Ok(receive_from_gen_channel)
    }
    
    /// Снять с регистрации
    pub fn deregister_udp_streaming(&self, client_addr: SocketAddr) -> Result<Option<SocketAddr>, QuoteError> {
        // Лочим зарегистрированных на запись, ошибку пробрасываем выше
        let mut streamers = self.udp_streamer_channels_rw.write()?;
        
        if let Some(_) = streamers.remove(&client_addr) {
            log::info!("🗙 UDP-стрим снят с подписок: {} ", client_addr);
            // Возвращаем Option с разрегистрированным клиентом
            Ok(Some(client_addr))
        } else {
            log::warn!("Попытка разрегистрации незарегистрированного UDP-стрима: {}", client_addr);
            // Такого клиент не зарегистрировано, выводим Warning, но это Ok
            Ok(None)
        }
    }

    // Вручную послать котировку клиенту
    pub fn send_quote_to_client(&self, client_addr: &SocketAddr, quote: StockQuote) -> Result<(), QuoteError> {
        let channels = self.udp_streamer_channels_rw.read()?;
        
        if let Some(client_channel) = channels.get(client_addr) {
            // Проверка, подписан ли на текущий тикер
            if client_channel.tickers.contains(&quote.ticker) {
                client_channel.to_streaming_thread_channel.send(quote.clone())
                    // Преобразуем ошибку работы с каналом в QuoteError::SendQuoteError 
                    .map_err(|send_error| {
                        let error_msg = format!("Ошибка отправки котировки '{}' для клиента {}: ошибка работы с каналом: {}", 
                                                         quote.ticker, client_addr, send_error);
                        QuoteError::SendQuoteError(error_msg)
                    })?;
                Ok(())
            } else {
                Err(QuoteError::SendQuoteError("UDP-стример не подписан на текущий тикер".to_string()))
            }
        } else {
            Err(QuoteError::SendQuoteError(format!("Клиент не найден {}", client_addr)))
        }
    }
    
    // Послать каждому подписчику котировки тикеров, на которые он подписан
    pub fn broadcast_quote(&self, quote: &StockQuote) -> Result<(), QuoteError> {
        let streamers = self.udp_streamer_channels_rw.read()?;
        
        for (_, client_channel) in streamers.iter() {
            if client_channel.tickers.contains(&quote.ticker) {
                let _ = client_channel.to_streaming_thread_channel.send(quote.clone());
            }
        }

        Ok(())
    }
    
    // Однократно послать котировки, на которые подписаны (основной цикл при создании потока)
    pub fn broadcast_quotes_to_subscribers(&self) -> Result<(), QuoteError> {
        let stock_prices = self.stock_prices_rw.read()?;

        for (ticker, _) in stock_prices.iter() {
            // Ошибку пробрасываем здесь
            match Self::generate_quote(self, ticker)? {
                Some(quote) => {
                    if let Err(e) = self.broadcast_quote(&quote) { 
                        log::error!("Не удалось отправить котировку [{}]: {}", ticker, e);
                    }
                },
                None => {
                    // Это маловероятно
                    log::error!("Отсутствует котировка для [{}]", ticker);
                }
            }


        }

        std::thread::sleep(Duration::from_millis(100));

        Ok(())
    }

    // Проверка, начался ли уже стриминг на указанный адрес:порт
    fn is_streaming_already_started(&self, client_addr: &SocketAddr, 
                                    streamers: &std::sync::RwLockWriteGuard<'_, HashMap<SocketAddr, UdpStreamSubscriber>>)
                                     -> Result<(), QuoteError> 
    {
        // Лок делаю на запись при попытке регистрации
        // let streamers = self.udp_streamer_channels_rw.read().unwrap();
        
        if streamers.contains_key(client_addr) {
            return Err(QuoteError::ClientAlreadyRegistered(client_addr.clone()));
        }

        Ok(())
    }
} 