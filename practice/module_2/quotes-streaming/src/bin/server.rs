// Запуск сервера - запускается на порту 11000:
// cargo run --bin server

use quotes_streaming::{SERVER_ADDR, PING_TIMEOUT_MILLISECS};
use quotes_streaming::quotes::{QuoteGenerator};

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

fn main() -> std::io::Result<()> {
    
    /* 1. Запускаем и инициализируем генератор котировок в отдельном потоке */
    
    // Новый генератор через Arc для передачи в потоки
    let quotes_generator_arc = Arc::new(QuoteGenerator::new());
    let quotes_generator_clone = quotes_generator_arc.clone();

    // Запускаем поток генератора цен до парсинга команд
    std::thread::spawn(move || -> std::io::Result<()> {
        let loaded_count = quotes_generator_clone.load_tickers_from_file(Path::new("aux/tickers.txt"))?;
        println!("Загружено {} компаний для стриминга ...", loaded_count);

        loop {
            quotes_generator_clone.update_prices();
            quotes_generator_clone.broadcast_quotes_to_subscribers();

            std::thread::sleep(Duration::from_secs(2));
        }

        // Ok(())
    });

    /* 2. Слушаем и обрабатываем входящие соединения */
    let listener = TcpListener::bind(SERVER_ADDR)?;
    println!("Сервер слушает на порту 11000");

    // Фактически бесконечный цикл, при возникновении соединения создаёт новый сервер
    // (блокирующий вызов .incoming(), аналог accept() в цикле)
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(to_client_stream) => {
                // Клонируем на каждой итерации
                let quotes_generator_clone = quotes_generator_arc.clone();

                // Здесь: main создаёт соединения с обработкой клиента, то есть каждый клиент обрабатывается
                // в своём потоке, фактически каждое соединение
                // Но при этом vault - разделяемое между потоками через Arc::clone(&vault)
                thread::spawn(move || {
                    server_process_request(to_client_stream, quotes_generator_clone);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    // Подключение классическое:
    // > nc 127.0.0.1 11000

    Ok(())
}

pub fn server_process_request(stream: TcpStream, quotes_generator: Arc<QuoteGenerator>) {

    // let client_addr = match stream.peer_addr() {
    //     Ok(addr) => addr,
    //     Err(e) => {
    //         eprintln!("Failed to get peer address: {}", e);
    //         return;
    //     }
    // };
    
    // let server_addr = match stream.local_addr() {
    //     Ok(addr) => addr,
    //     Err(e) => {
    //         eprintln!("Failed to get local address: {}", e);
    //         return;
    //     }
    // };

    // TODO: обработать ошибки
    let client_addr = stream.peer_addr().expect("Не удалось получить адрес клиента из сокета");
    let server_addr = stream.local_addr().expect("Не удалось получить адрес сервера из сокета");

    // Клонируем stream: один экземпляр для чтения (обёрнут в BufReader), другой — для записи (для двух буферов под капотом)
    let mut to_client_tcp_stream = stream.try_clone().expect("Ошибка клонирования стрима");
    let mut to_server_tcp_stream = BufReader::new(stream);

    // Выводим серверу и отправляем клиенту
    println!("Подключение к серверу: {} => {}", client_addr, server_addr);

    let welcome_string = format!("Вы подключились к серверу: {} => {} \nДля начала стриминга введите команду: STREAM udp://host:port TICKER1,TICKER2\n",
                                                                     client_addr, server_addr
    );
    
    // Отправляем Welcome клиенту
    if let Err(e) = to_client_tcp_stream.write_all(welcome_string.as_bytes()) {
        eprintln!("Не удалось отправить Welcome-сообщение: {}", e);
        return;
    }
    let _ = to_client_tcp_stream.flush();

    let mut command_from_client = String::new();
    // let mut response = String::new();

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
                    let error_msg = "Вы ничего не ввели. Введите команду в формате: STREAM udp://host:port TICKER1,TICKER2\n".to_string();
                    let _ = to_client_tcp_stream.write_all(error_msg.as_bytes());
                    let _ = to_client_tcp_stream.flush();
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
                                let error_msg = format!("ERROR: Ошибка создания UDP-сокета для клиента: {}\n", e);
                                let _ = to_client_tcp_stream.write_all(error_msg.as_bytes());
                                let _ = to_client_tcp_stream.flush();
                                continue;
                            }
                        };
                        
                        // Адрес:порт UDP-стриминга
                        let server_udp_addr_port = match server_udp_socket.local_addr() {
                            Ok(addr) => addr,
                            Err(e) => {
                                let error_msg = format!("ERROR: Ошибка работы с сокетом: {}\n", e);
                                let _ = to_client_tcp_stream.write_all(error_msg.as_bytes());
                                let _ = to_client_tcp_stream.flush();
                                continue;
                            }
                        };
                        
                        // Регистрация клиента
                        let quotes_generator_clone = quotes_generator.clone();
                        let read_from_gen_channel = match quotes_generator_clone.register_udp_streaming(
                                                                    stream_command_ok.client_udp_addr, stream_command_ok.tickers) 
                        {
                            Ok(receiver) => receiver,
                            Err(e) => {
                                let error_msg = format!("ERROR: Ошибка регистрации подписки на тикеры: {}\n", e);
                                let _ = to_client_tcp_stream.write_all(error_msg.as_bytes());
                                let _ = to_client_tcp_stream.flush();
                                continue;
                            }
                        };

                        let success_msg = format!("OK: Начало стриминга: {} → {}\n", server_udp_addr_port, stream_command_ok.client_udp_addr);
                        let _ = to_client_tcp_stream.write_all(success_msg.as_bytes());
                        let _ = to_client_tcp_stream.flush();

                        println!("Начало стриминга: {} → {}", server_udp_addr_port, stream_command_ok.client_udp_addr);

                        /*** Создание потоков: UDP-стриминг и PING ***/

                        // Сокет для пинга на чтение. TODO: обработка unwrap(), всё понимаю
                        let ping_udp_socket = server_udp_socket.try_clone().unwrap();

                        // Делится между потоками, делаем по паре
                        let last_ping = Arc::new(AtomicU64::new(now_milliseconds()));
                        let last_ping_clone = Arc::clone(&last_ping);
                        
                        let stop_streaming = Arc::new(AtomicBool::new(false));
                        let stop_streaming_for_ping = Arc::clone(&stop_streaming);

                        // Поток-получатель PING'а
                        // (не уверен, что это лучшая идея, но пока как есть. Может, лучше в самом стриминг-потоке сделать, не понял, разрешает ли это задание)
                        thread::spawn(move || {
                            // Аналогично клиенту, читающему UDP-датаграммы
                            ping_udp_socket.set_read_timeout(Some(Duration::from_millis(100))).ok();
                            let mut buffer = [0u8; 32];

                            loop {
                                // Здесь и ниже использую Relaxed - не нужен строгий порядок memory_order
                                if stop_streaming_for_ping.load(Ordering::Relaxed) == true { 
                                    break; 
                                }

                                match ping_udp_socket.recv_from(&mut buffer) {
                                    Ok((len, _)) => {
                                        if let Ok(msg) = std::str::from_utf8(&buffer[..len]) {
                                            if msg.trim() == "PING" {
                                                last_ping_clone.store(now_milliseconds(), Ordering::Relaxed);
                                            }
                                        }
                                    }
                                    Err(e) if e.kind() == std::io::ErrorKind::TimedOut 
                                                  || e.kind() == std::io::ErrorKind::WouldBlock => {}
                                    Err(e) => eprintln!("Ping recv error: {}", e),
                                }

                                
                                // Сравниваем с атомарной (разница до нуля)
                                let now = now_milliseconds();
                                let last = last_ping_clone.load(Ordering::Relaxed);

                                if now.saturating_sub(last) > PING_TIMEOUT_MILLISECS {
                                    println!("Таймаут для {}. Останавливаем стриминг", stream_command_ok.client_udp_addr);
                                    // Бабахаем остановку в потоке стриминга
                                    stop_streaming_for_ping.store(true, Ordering::Relaxed);
                                    break;
                                }
                            }
                            
                            println!("Завершение PING-обработчика для {}", stream_command_ok.client_udp_addr);
                        });

                        // Поток-получатель PING'а, просто без закрытия стриминга (не уверен, что это лучшая идея, но пока как есть)
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
                        thread::spawn(move || {
                            // Читаем из каналов от генератора

                            loop {
                                    // Check stop flag frequently
                                    if stop_streaming.load(Ordering::Relaxed) {
                                        // Снимаем регистрацию
                                        quotes_generator_clone.deregister_udp_streaming(stream_command_ok.client_udp_addr);
                                        
                                        println!("Получен сигнал остановки для {}", stream_command_ok.client_udp_addr);
                                        break;
                                    }

                                    // Чтение с таймаутом, здесь можно через один вызов
                                    match read_from_gen_channel.recv_timeout(Duration::from_millis(50)) {
                                        Ok(read_quote) => {
                                            let quote_as_string = format!("{}\n", read_quote.to_string());

                                            if let Err(e) = server_udp_socket.send_to(quote_as_string.as_bytes(), 
                                                                                             stream_command_ok.client_udp_addr) {
                                                eprintln!("Ошибка отправки {}: {}", stream_command_ok.client_udp_addr, e);
                                                break;
                                            }
                                            // Отправилос
                                            println!("📤 Отправлено: {} [{}] → {}",server_udp_addr_port,
                                                                                    read_quote.ticker, 
                                                                                    stream_command_ok.client_udp_addr);
                                        }
                                        // Аналогично - обработка для неблокирующего вызова
                                        Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                                        Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
                                    }
                                }

                                println!("Завершение UDP-стримера для {}", stream_command_ok.client_udp_addr);

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
                    },
                    Err(parse_error) => {
                        let error_msg = format!("ERROR: {}\n", parse_error);
                        let _ = to_client_tcp_stream.write_all(error_msg.as_bytes());
                        let _ = to_client_tcp_stream.flush();
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
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
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
            return Err("Неверный формат команды: STREAM udp://host:port TICKER1,TICKER2".into());
        }
        
        // Отрезаем префикс
        let addr_str_no_prefix = parts[1].strip_prefix("udp://")
            .ok_or("Принимаем только UDP: в адресе нет префикса udp://")?;

        // Парсим адрес с портом через parse<SocketAddr>
        let client_udp_addr = addr_str_no_prefix.parse::<SocketAddr>()
            .map_err(|e| format!("Адрес не распарсился {}: {}", addr_str_no_prefix, e))?;
        
        // Разделяем компании, засовываем в вектор (совсем путые тикеры тоже убираем)
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