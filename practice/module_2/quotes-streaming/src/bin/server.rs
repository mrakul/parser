use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;
use std::io::Write;
use std::net::{TcpStream, TcpListener};
use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::net::SocketAddr;
use rand::Rng;


use quotes_streaming::quotes::{StockQuote, QuoteGenerator};

fn main() -> std::io::Result<()> {
    
    let server_addr_port = "127.0.0.1:11000";
    let listener = TcpListener::bind(server_addr_port)?;

    println!("Server listening on port 11000");

    // Новое хранилище, обёрнутое в мьютекс и Arc
    let quotes_generator = Arc::new(QuoteGenerator::new());

    // Фактически бесконечный цикл, при возникновении соединения создаёт новый сервер
    // (блокирующий вызов .incoming(), аналог accept() в цикле)
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(to_client_stream) => {
                // Здесь: main создаёт соединения с обработкой клиента, то есть каждый клиент обрабатывается
                // в своём потоке, фактически каждое соединение
                // Но при этом vault - разделяемое между потоками через Arc::clone(&vault)
                thread::spawn(move || {
                    server_process_request(to_client_stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    // Подключение классическое:
    // > nc 127.0.0.1 7878

    Ok(())
}

pub fn server_process_request(stream: TcpStream) {

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

    let client_addr = stream.peer_addr().expect("failed to get client address");
    let server_addr = stream.local_addr().expect("failed to get server address");

    // клонируем stream: один экземпляр для чтения (обёрнут в BufReader), другой — для записи
    let mut to_client_stream = stream.try_clone().expect("failed to clone stream");
    let mut to_server_stream = BufReader::new(stream);

    let welcome_string = format!("Вы подключились к серверу: {} => {} \nВведите команду: STREAMING, PING \n> ",
                                                                     client_addr, server_addr
    );
    
    // Отправляем Welcome клиенту
    if let Err(e) = to_client_stream.write_all(welcome_string.as_bytes()) {
        eprintln!("Не удалось отправить Welcome-сообщение: {}", e);
        return;
    }
    let _ = to_client_stream.flush();

    let mut command_from_client = String::new();
    let mut response = String::new();

    loop {
        // Очищаем входную строку и главное - response        
        command_from_client.clear();
        response.clear();

        // read_line ждёт '\n' — nc отправляет строку по нажатию Enter
        match to_server_stream.read_line(&mut command_from_client) {
            Ok(0) => {
                // EOF — клиент закрыл соединение
                return;
            }
            // Успешно прочитали line
            Ok(_) => {

                // Пустой ввод
                let command_from_client = command_from_client.trim();
                if command_from_client.is_empty() {
                    response = "Вы ничего не ввели. Введите команду в формате: STREAM udp://host:port TICKER1,TICKER2\n>".to_string();
                    let _ = to_client_stream.write_all(response.as_bytes());
                    let _ = to_client_stream.flush();

                    let _ = to_client_stream.flush();
                    continue;
                }

                match StreamCommand::parse(&command_from_client) {
                    Ok(stream_command_ok ) => {
                        // Создание нового соединения UDP

                    },
                    Err(e) => {
                        response = e;
                    }
                }
                
                // Отправляем ответ и снова показываем промпт
                let _ = to_client_stream.write_all(response.as_bytes());
                let _ = to_client_stream.flush();
            }
            Err(_) => {
                // ошибка чтения — закрываем
                return;
            }
        }
    }
}


/*** Секция команды STREAMING ***/

pub struct StreamCommand {
    pub client_addr_port: SocketAddr,
    pub companies: Vec<String>,
}

impl StreamCommand {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 3 || parts[0].to_uppercase() != "STREAM" {
            return Err("ERROR: Неверный формат команды: STREAM udp://host:port TICKER1,TICKER2".into());
        }
        
        let addr_str_no_prefix = parts[1].strip_prefix("udp://")
            .ok_or("ERROR: Принимаем только UDP: в адресе нет префикса udp://")?;

        let target_addr = addr_str_no_prefix.parse::<SocketAddr>()
            .map_err(|e| format!("Адрес не распарсился {}: {}", addr_str_no_prefix, e))?;
        
        let companies: Vec<String> = parts[2].split(',')
            .map(|t| t.trim().to_uppercase())
            .filter(|t| !t.is_empty())
            .collect();
        
        if companies.is_empty() {
            return Err("ERROR: Не указано ни одной компании".into());
        }
        
        Ok(Self{client_addr_port: target_addr, companies})
    }
}