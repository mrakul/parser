// Запуск сервера - запускается на порту 11000:
// cargo run --bin server

use crossbeam_channel::Receiver;
use quotes_streaming::{SERVER_ADDR, PING_TIMEOUT_MILLISECS};
use quotes_streaming::quotes::{QuoteError, QuoteGenerator, StockQuote};

use std::io::{BufRead, BufReader};
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io::Write;
use std::net::{TcpStream, TcpListener};
use std::sync::Arc;
use std::thread;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;

fn main() -> std::io::Result<()> {
    /* 0. Инициализируем логгер */
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("debug")
    ).init();
    
    log::info!("🚀 Старт сервера ...");

    /* 1. Запускаем и инициализируем генератор котировок в отдельном потоке */
    
    // Новый генератор через Arc для передачи в потоки
    let quotes_generator_arc = Arc::new(QuoteGenerator::new());
    let quotes_generator_clone = quotes_generator_arc.clone();

    // Запускаем поток генератора цен до парсинга команд
    std::thread::spawn(move || -> Result<(), QuoteError> {
        let loaded_count = quotes_generator_clone.load_tickers_from_file(Path::new("aux/tickers.txt"))?;
        log::info!("Загружено {} компаний для стриминга ...", loaded_count);

        loop {
            if let Err(error) = quotes_generator_clone.update_prices() {
                log::error!("Не удалось обновить цены: {}", error);
                // Если критическая ошибка, останавливаем обработку. Пока оставляю так
                // break; 
            }

            if let Err(error) = quotes_generator_clone.broadcast_quotes_to_subscribers() {
                log::error!("Не удалось послать котировки подписчикам: {}", error);
                // Аналогично
                // break;
            }

            std::thread::sleep(Duration::from_secs(2));
        }
    });

    /* 2. Слушаем и обрабатываем входящие соединения */
    let listener = TcpListener::bind(SERVER_ADDR)?;
    log::info!("Сервер слушает на порту 11000");

    // Фактически бесконечный цикл, при возникновении соединения создаёт новый сервер
    // (блокирующий вызов .incoming(), аналог accept() в цикле)
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(to_client_stream) => {
                // Клонируем на каждой итерации
                let quotes_generator_clone = quotes_generator_arc.clone();

                thread::spawn(move || {
                    server_process_request(to_client_stream, quotes_generator_clone);
                });
            }
            Err(e) => log::error!("Connection failed: {}", e),
        }
    }

    // Подключение классическое:
    // > nc 127.0.0.1 11000

    Ok(())
}


pub fn server_process_request(mut stream: TcpStream, quotes_generator: Arc<QuoteGenerator>) {

    let client_addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(error) => {
            send_msg_to_tcp_client(&mut stream, format!("Ошибка работы с сокетом {}\n", error));
            log::error!("Ошибка работы с сокетом {}", error);
            return;
        }
    };
    
    let server_addr = match stream.local_addr() {
        Ok(addr) => addr,
        Err(error) => {
            send_msg_to_tcp_client(&mut stream, format!("Ошибка работы с сокетом {}\n", error));
            log::error!("Ошибка работы с сокетом {}", error);
            return;
        }
    };

    // Клонируем stream: один экземпляр для чтения (обёрнут в BufReader), другой — для записи (для двух буферов под капотом)
    let mut to_client_tcp_stream = match stream.try_clone() {
        Ok(stream) => stream,
        Err(e) => {
            log::error!("Ошибка клонирования stream: {}", e);
            // Клиент должен получить TCP FIN
            return;
        }
    };
    let mut to_server_tcp_stream = BufReader::new(stream);

    // Выводим серверу и отправляем клиенту
    log::info!("Подключение к серверу: {} => {}", client_addr, server_addr);

    let welcome_string = format!("Вы подключились к серверу: {} => {} \nДля начала стриминга введите команду: STREAM udp://host:port TICKER1,TICKER2\n",
                                                                     client_addr, server_addr
    );
    
    // Отправляем Welcome клиенту
    if let Err(e) = to_client_tcp_stream.write_all(welcome_string.as_bytes()) {
        log::error!("Не удалось отправить Welcome-сообщение: {}", e);
        return;
    }
    let _ = to_client_tcp_stream.flush();

    // Читаем команду в String
    let mut command_from_client = String::new();

    loop {
        // Очищаем входную строку и главное - response        
        command_from_client.clear();
        // response.clear();

        // read_line ждёт '\n' — nc отправляет строку по нажатию Enter
        match to_server_tcp_stream.read_line(&mut command_from_client) {
            Ok(0) => {
                // EOF — клиент закрыл соединение
                return;
            }
            // Успешно прочитали line
            Ok(_) => {

                // Пустой ввод
                let command_from_client = command_from_client.trim();
                if command_from_client.is_empty() {
                    send_msg_to_tcp_client(&mut to_client_tcp_stream, 
                                   "Вы ничего не ввели. Введите команду в формате: STREAM udp://host:port TICKER1,TICKER2\n".to_string());
                    continue;
                }

                match StreamCommand::parse(&command_from_client) {
                    Ok(stream_command_ok ) => {
                        // Создание сокета UDP для стриминга: использую парные порты, можно согласовать по TCP, если требуется
                        let server_udp_socket_result = UdpSocket::bind(format!("127.0.0.1:{}",
                                                                                    stream_command_ok.client_udp_addr.port() + 1));
                        
                        let server_udp_socket = match server_udp_socket_result {
                            Ok(socket) => socket,
                            Err(e) => {
                                log::error!("ERROR: Ошибка создания UDP-сокета для клиента: {}", e);
                                send_msg_to_tcp_client(&mut to_client_tcp_stream,
                                                                format!("ERROR: Ошибка создания UDP-сокета для клиента: {}\n", e));
                                continue;
                            }
                        };
                        
                        // Адрес:порт UDP-стриминга
                        let server_udp_addr_port = match server_udp_socket.local_addr() {
                            Ok(addr) => addr,
                            Err(e) => {
                                log::error!("ERROR: Ошибка работы с сокетом: {}", e);
                                send_msg_to_tcp_client(&mut to_client_tcp_stream,
                                                                format!("ERROR: Ошибка работы с сокетом: {}\n", e));
                                continue;
                            }
                        };
                        
                        // Регистрация клиента
                        // let quotes_generator_clone = quotes_generator.clone();
                        let read_from_gen_channel = match quotes_generator.register_udp_streaming(
                                                                    stream_command_ok.client_udp_addr, stream_command_ok.tickers) 
                        {
                            Ok(receiver) => receiver,
                            Err(e) => {
                                log::error!("ERROR: Ошибка регистрации подписки на тикеры: {}", e);
                                send_msg_to_tcp_client(&mut to_client_tcp_stream,
                                                                format!("ERROR: Ошибка регистрации подписки на тикеры: {}\n", e));
                                continue;
                            }
                        };

                        send_msg_to_tcp_client(&mut to_client_tcp_stream,
                                                        format!("OK: Начало стриминга: {} → {}\n", server_udp_addr_port, 
                                                                                                   stream_command_ok.client_udp_addr));
                        log::info!("Начало стриминга: {} → {}", server_udp_addr_port, stream_command_ok.client_udp_addr);

                        /*** Создание потоков: UDP-стриминг и PING ***/

                        // Сокет для пинга на чтение. TODO: обработка unwrap(), всё понимаю
                        let ping_udp_socket = match server_udp_socket.try_clone() {
                            Ok(socket) => socket,
                            Err(e) => {
                                log::error!("Ошибка клонирования для PING-receiver'а: {} {}", e, stream_command_ok.client_udp_addr);
                                return;
                            }
                        };

                        // Делится между потоками, делаем по паре (клон при передаче в функцию)

                        // Пинг Arc'ованный не нужен в текущем подходе, используется только PING-приёмником с одной moved-версией при создании потока
                        let last_ping = AtomicU64::new(now_milliseconds());
                        let stop_streaming_flag = Arc::new(AtomicBool::new(false));
                        // let last_ping = Arc::new(AtomicU64::new(now_milliseconds()));

                        // Запускакем UDP-стриминговый поток для клиента
                        launch_udp_streamer(server_udp_socket, 
                                            server_udp_addr_port,
                                            quotes_generator.clone(), 
                                            read_from_gen_channel,
                                            Arc::clone(&stop_streaming_flag), 
                                            stream_command_ok.client_udp_addr);

                        // Поток-получатель PING'а
                        launch_udp_ping_receiver(ping_udp_socket, 
                                                    last_ping, 
                                                    stop_streaming_flag, 
                                                    stream_command_ok.client_udp_addr);
                    },
                    Err(parse_error) => {
                        log::error!("ERROR: {}\n", parse_error);
                        send_msg_to_tcp_client(&mut to_client_tcp_stream,
                                                        format!("ERROR: {}\n", parse_error));
                    }
                }
            }
            Err(_) => {
                // ошибка чтения — закрываем
                return;
            }
        }
    }
}

fn now_milliseconds() -> u64 {
    // Оставил .unwrap() намеренно, поскольку крайне меловероятно, и здесь это подходящий подход, насколько понял
    // Но суть ошибки понял, если текущая дата установлена меньше UNIX_EPOCH (можно поменять на expect())
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

// Отправка клиенту сообщения по TCP
fn send_msg_to_tcp_client(to_client_stream: &mut TcpStream, message: String) {
    let _ = to_client_stream.write_all(message.as_bytes());
    // TODO: можно обработать ошибку
    let _ = to_client_stream.flush();
}

/*** Секция команды STREAM ***/

pub struct StreamCommand {
    pub client_udp_addr: SocketAddr,
    pub tickers: Vec<String>,
}

impl StreamCommand {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        // Проверяю тут жёстко на конкретную команду (можно отдельно, если буду добавлять команды)
        if parts.len() != 3 || parts[0].to_uppercase() != "STREAM" {
            // ERROR подставляем верхнеуровнево. (!) И newline там же
            log::warn!("Неверный формат команды: STREAM udp://host:port TICKER1,TICKER2\n");
            return Err("Неверный формат команды: STREAM udp://host:port TICKER1,TICKER2".into());
        }
        
        // Отрезаем префикс
        let addr_str_no_prefix = match parts[1].strip_prefix("udp://") {
            Some(addr_no_prefix) => addr_no_prefix,
            None => {
                log::warn!("Принимаем только UDP: в адресе нет префикса udp://");
                return Err("Принимаем только UDP: в адресе нет префикса udp://".into());
            }
        };
            

        // Парсим адрес с портом через parse<SocketAddr>
        let client_udp_addr = match addr_str_no_prefix.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(e) => {
                log::warn!("Адрес не распарсился {}: {}", addr_str_no_prefix, e);
                return Err(format!("Адрес не распарсился {}: {}", addr_str_no_prefix, e));
            }
        };
        // .map_err(|e| format!("Адрес не распарсился {}: {}", addr_str_no_prefix, e))?;
        
        // Разделяем компании, засовываем в вектор (совсем пуcтые тикеры тоже убираем)
        let tickers: Vec<String> = parts[2].split(',')
            .map(|t| t.to_uppercase())
            .filter(|t| !t.is_empty())
            .collect();
        
        if tickers.is_empty() {
            return Err("Не указано ни одной компании".into());
        }
        
        Ok(Self{client_udp_addr, tickers})
    }
}

/*** Создание потоков ***/

fn launch_udp_ping_receiver(from_client_udp_socket: UdpSocket,
                            last_ping: AtomicU64,
                            stop_streaming_flag: Arc::<AtomicBool>,
                            to_client_udp_addr: SocketAddr) -> JoinHandle<()>
{
    let join_handle = thread::spawn(move || {
        // Аналогично клиенту, читающему UDP-датаграммы
        if let Err(e) = from_client_udp_socket.set_read_timeout(Some(Duration::from_millis(100))) {
            log::warn!("Невозможность установки таймаута для сокета: {}", e);
            // Поставлю ворнинг, но сокет может заблокироваться дольше ожидаемого
        }

        let mut buffer = [0u8; 32];

        loop {
            // Здесь и ниже использую Relaxed - не нужен строгий порядок memory_order
            if stop_streaming_flag.load(Ordering::Relaxed) == true { 
                break; 
            }

            match from_client_udp_socket.recv_from(&mut buffer) {
                Ok((len, _)) => {
                    if let Ok(msg) = std::str::from_utf8(&buffer[..len]) {
                        if msg.trim() == "PING" {
                            last_ping.store(now_milliseconds(), Ordering::Relaxed);
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut 
                                || e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => log::error!("Ошибка получения пинга: {}", e),
            }

            
            // Сравниваем с атомарной (разница до нуля)
            let now = now_milliseconds();
            let last = last_ping.load(Ordering::Relaxed);

            if now.saturating_sub(last) > PING_TIMEOUT_MILLISECS {
                log::warn!("Таймаут для {}. Останавливаем стриминг", to_client_udp_addr);
                // Бабахаем остановку в потоке стриминга
                stop_streaming_flag.store(true, Ordering::Relaxed);
                break;
            }
        }
        
        log::info!("Завершение PING-обработчика для {}", to_client_udp_addr);
    });

    join_handle
}

// TEMP:
// Поток-получатель PING'а, просто без закрытия стриминга
// thread::spawn(move || {
//     let mut buffer = [0u8; 32];
//     loop {
//         match ping_udp_socket.recv_from(&mut buffer) {
//             Ok((bytes_read, client_addr)) => {
//                 let msg = std::str::from_utf8(&buffer[..bytes_read]).unwrap_or("");
//                 if msg.trim() == "PING" {
//                     println!("🏓 PING получен от {}", client_addr);
//                     // TODO: логика апдейта и завершения стриминга, если таймаут 
//                 }
//             }
//             Err(e) => eprintln!("Ping receive error: {}", e),
//         }
//     }
// });


// Поток UDP-стриминга под нового клиента 
fn launch_udp_streamer(to_client_udp_socket: UdpSocket,
                       server_udp_addr_port: SocketAddr,
                       quotes_generator: Arc::<QuoteGenerator>,
                       read_from_gen_channel: Receiver<StockQuote>,
                       stop_streaming_flag: Arc::<AtomicBool>,
                       to_client_udp_addr: SocketAddr) -> JoinHandle<()>
{
    let join_handle = thread::spawn(move || {
        // Читаем из каналов от генератора

        log::debug!("Создание потока стриминга для {}", to_client_udp_addr);

        loop {
                // Проверка атомарной переменной
                if stop_streaming_flag.load(Ordering::Relaxed) == true {
                    // Снимаем регистрацию. Успешные логируютс внутри deregister
                    if let Err(error) =  quotes_generator.deregister_udp_streaming(to_client_udp_addr) {
                        log::error!("Ошибка снятия с подписки {}: {}", to_client_udp_addr, error);
                    }

                    
                    log::warn!("Получен сигнал остановки для {}", to_client_udp_addr);
                    break;
                }

                // Чтение с таймаутом, здесь можно через один вызов
                match read_from_gen_channel.recv_timeout(Duration::from_millis(50)) {
                    Ok(read_quote) => {
                        let quote_as_string = format!("{}\n", read_quote.to_string());

                        if let Err(e) = to_client_udp_socket.send_to(quote_as_string.as_bytes(), 
                                                                  to_client_udp_addr) 
                        {
                            log::error!("Ошибка отправки {}: {}", to_client_udp_addr, e);
                            break;
                        }
                        // Отправилос
                        log::debug!("📤 Отправлено: {} [{}] → {}",server_udp_addr_port,
                                                                  read_quote.ticker, 
                                                                  to_client_udp_addr);
                    }
                    // Аналогично - обработка для неблокирующего вызова
                    Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                    Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
                }
            }

            log::info!("Завершение UDP-стримера для {}", to_client_udp_addr);
        

        // TEMP: Без обработки атомарного флага
        // loop {
        //     match read_from_gen_channel.recv() {
        //         Ok(read_quote) => {
        //             let data = read_quote.to_string() + "\n";
        //             if let Err(e) = server_udp_socket.send_to(data.as_bytes(), stream_command_ok.client_addr_port) {
        //                 eprintln!("Ошибка отправки {}: {}", stream_command_ok.client_addr_port, e);
        //                 break;
        //             }
        //             println!("📤 Отправлено: {} [{}] → {}",server_udp_addr_port,
        //                                                                    read_quote.ticker, 
        //                                                                    stream_command_ok.client_addr_port);
        //         }
        //         Err(_) => {
        //             // Ошибка чтения из канала
        //             println!("Ошибка чтения из канала {}", stream_command_ok.client_addr_port);
        //             continue;
        //         }
        //     }

        //     // Пусть пока читают сразу по несколько тиков
        //     thread::sleep(Duration::from_millis(500));
        // }
        
        // // TEMP: Предыдущая реализация напрямую через generator с локами
        // loop {
        //     for ticker in &stream_command_ok.tickers {
        //         // Генерируем нужную котировку
        //         if let Some(quote) = quotes_generator_clone.generate_quote(ticker) {
        //             let data = quote.to_string() + "\n";
        //             // Посылаем на адрес из команды
        //             if let Err(e) = server_udp_socket.send_to(data.as_bytes(), stream_command_ok.client_addr_port) {
        //                 eprintln!("Ошибка отправки {}: {}", stream_command_ok.client_addr_port, e);
        //                 // Ошибка при посылке
        //                 break;
        //             }
        //             println!("📤 Отправлено: {} → {}", quote.ticker, stream_command_ok.client_addr_port);
        //         }
        //     }

        //      thread::sleep(Duration::from_secs(10));
        // }
    });

    join_handle
}
