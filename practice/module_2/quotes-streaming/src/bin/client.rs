// Запуск клиента, пример:
// > cargo run --bin client -- --server-addr-port 127.0.0.1:11000 --udp-client-port 30000 --subscriptions-file aux/client_1_tickers.txt
//
//  --server-addr-port <SERVER_ADDR_PORT>
//  --udp-client-port <UDP_CLIENT_PORT>
//  --subscriptions-file <SUBSCRIPTIONS_FILE>

// Пускануть два клиента со своими подписками:
// cargo run --bin client -- --server-addr-port 127.0.0.1:11000 --udp-client-port 30000 --subscriptions-file aux/client_1_tickers.txt
// cargo run --bin client -- --server-addr-port 127.0.0.1:11000 --udp-client-port 30002 --subscriptions-file aux/client_2_tickers.txt

use quotes_streaming::PING_INTERVAL_SECS;

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpStream, UdpSocket, SocketAddr};
use std::fs::File;
use std::thread::JoinHandle;
use std::time::Duration;
use clap::{Parser};
// Для завершения по Ctrl+C сигналу
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Parser, Debug)]
#[command(name = "Клиент получения котировок с заданием нужных котировок")]
struct Args {
    #[arg(long)]
    server_addr: String,

    #[arg(long)]
    udp_port: String,

    #[arg(long)]
    tickers_file: String,

}

fn main() -> io::Result<()> {

    // Используем атомарную переменную для обработки SIGINT
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let shutdown_flag_clone = Arc::clone(&shutdown_flag);

    // Обработчик SIGINT
    if let Err(error) = ctrlc::set_handler(move || {
        println!("\n Получен Ctrl+C SIGINT, закрываем потоки ...");
        shutdown_flag_clone.store(true, Ordering::SeqCst);
    }) 
    {
        println!("Ошибка установки обработчика SIGINT: {}", error);
        std::process::exit(1);
    };

    // Читаем аргументы командной строки, ошибки формата обрабатываются в parse()
    let args = Args::parse();

    let subscriptions_open = File::open(args.tickers_file)?;
    let reader_subs = BufReader::new(subscriptions_open);
    let mut subscriptions= Vec::<String>::new();

    // Заполняем подписки
    for line in reader_subs.lines() {
        let line = line?;
        let cur_ticker = line.trim();
        
        subscriptions.push(cur_ticker.to_string());
    }
    
    println!("Запрошенные котировки: {:?}", subscriptions);

    // Парсим адрес
    let server_addr_port= match args.server_addr.parse::<SocketAddr>() {
        Ok(parsed_addr) => parsed_addr,
        Err(error) => {
            eprintln!("Адрес не распарсился '{}': {}", args.server_addr, error);
            std::process::exit(1);
        }
        // Можно .unwrap_or_else()
    };

    // Парсим UDP-порт, куда будут приходит котировки
    let udp_client_port: u16 = match args.udp_port.parse::<u16>() {
        Ok(parsed_port) => parsed_port,
        Err(error) => {
            eprintln!("Порт не распарсился '{}': {}", args.udp_port, error);
            std::process::exit(1);
        }
    };
    
    // Подключаемся и клонируем stream для reader'а
    println!("Подключаемся к {}", server_addr_port);

    let to_server_stream = TcpStream::connect(server_addr_port)?;
    let mut to_client_stream = BufReader::new(to_server_stream.try_clone()?);

    // Читаем приветствие
    let mut buffer = [0; 1024];
    let read_count = to_client_stream.read(&mut buffer)?;
    let starting_string = String::from_utf8_lossy(&buffer[..read_count]);
    print!("{}", starting_string);

    if let Err(e) = io::stdout().flush() {
        eprintln!("Warning: проблема вывода в stdout: {}", e);
        // Пока ворнинг - может быть некритично для основной логики
    }

    // Посылаем команду STREAM: STREAM udp://127.0.0.1:30000 [тикеры из файла]
    let stream_request_command = format!("STREAM udp://127.0.0.1:{} {}", udp_client_port, subscriptions.join(","));
    println!("Посылаемая команда: {}", stream_request_command);

    // Создание сокета UDP для стриминга
    let client_udp_socket = UdpSocket::bind(format!("127.0.0.1:{}", udp_client_port))
        .unwrap_or_else(|error| {
            eprintln!("Не удалось получить сокет для UDP-приёма котировок на порт {}: {}", format!("127.0.0.1:{}", udp_client_port), error);
            std::process::exit(1);
        });

        match client_udp_socket.local_addr() {
            Ok(addr) => println!("Адрес UDP-приёмника котировок: {}", addr),
            Err(e) => {
                eprintln!("Не удалось получить адрес локального UDP сокета: {}", e);
                std::process::exit(1);
            }
        }

    // Handle'ы для потоков receiver'а, ping'ера
    let mut udp_receiver_join_handle_opt: Option<JoinHandle<()>> = None;
    let mut udp_ping_join_handle_opt: Option<JoinHandle<()>> = None;

    match send_command(&to_server_stream, &mut to_client_stream, &stream_request_command) {
        Ok(response) => {
            // Парсинг ответа
            print!("{}", response);
            // Разбираем команду
            let reponse_tokens: Vec<&str> = response.split_whitespace().collect();

            if reponse_tokens.first() == Some(&"OK:") {
                // Поток-приёмник котировок
                udp_receiver_join_handle_opt = Some(launch_udp_receiver(client_udp_socket.try_clone()?, Arc::clone(&shutdown_flag)));
                
                // Поток-пингователь
                let mut udp_ping_server_addr = server_addr_port.clone();
                udp_ping_server_addr.set_port(udp_client_port + 1);
                udp_ping_join_handle_opt = Some(launch_udp_ping(client_udp_socket, udp_ping_server_addr, shutdown_flag));
            }
            else if reponse_tokens.first() == Some(&"ERROR:") {
                println!("Произошла ошибка при подключении к серверу: {}", response);
                std::process::exit(1);
            }
        }
        Err(e) => {
            println!("ERROR: Сервер закрыл соединение ({})", e);
        }
    }

    // Ждём завершение, если был запущен
    if let Some(udp_receiver_join_handle) = udp_receiver_join_handle_opt  {
        if let Err(e) = udp_receiver_join_handle.join() {
            eprintln!("Поток UDP-receiver не завершился успешно: {:?}", e);
        }
    }
    
    if let Some(udp_ping_join_handle) = udp_ping_join_handle_opt  {
        if let Err(e) = udp_ping_join_handle.join() {
            eprintln!("Поток UDP-ping не завершился успешно: {:?}", e);
        }
    }

    Ok(())

}

// Отправка команды по TCP-стриму и получение response в BufReader
fn send_command(
    mut stream: &TcpStream,
    reader: &mut BufReader<TcpStream>,
    command: &String,
) -> io::Result<String> 
{
    stream.write_all(command.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    let mut read_buffer = String::new();
    let bytes_read_cnt = reader.read_line(&mut read_buffer)?;

    if bytes_read_cnt == 0 {
        // Cервер закрыл соединение
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof,
                                 "Cервер закрыл соединение",
        ));
    }

    Ok(read_buffer)
}

fn _append_tickers(command: &mut String, tickers_vec: Vec<String>) {
    for cur_ticker in &tickers_vec {
        command.push_str(cur_ticker); 
    }
}

// Запуск UDP-ресивера
fn launch_udp_receiver(udp_socket: UdpSocket, shutdown_flag: Arc<AtomicBool>) -> JoinHandle<()> {
        // Поток UDP-стриминга под нового клиента 
    let udp_receiver_join_handle = std::thread::spawn(move || {
            // Читаем из каналов
            let mut buffer = [0u8; 1024];
    
            // Чтобы можно было обработать Ctrl+C не "под нагрузкой" потока
            if let Err(e) = udp_socket.set_read_timeout(Some(std::time::Duration::from_millis(100))) {
                eprintln!("Warning: невозможность установки таймаута для сокета: {}", e);
            }

            while !shutdown_flag.load(Ordering::SeqCst) {
                match udp_socket.recv_from(&mut buffer) {
                    Ok((bytes_read, sender_addr)) => {
                        // Ожидаем символы UTF-8
                        match std::str::from_utf8(&buffer[..bytes_read]) {
                            Ok(message) => {
                                println!("🚀 Получено от {}: {}", sender_addr, message.trim());
                            }
                            Err(error) => eprintln!("Не удалось декодировать UTF-8: {}", error),
                        }
                    },
                    // Надо проверить, в противном случае recv_from блокирующий, то есть флаг сработает только "под нагрузкой"
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock || 
                                     e.kind() == std::io::ErrorKind::TimedOut => {
                        continue;
                    },
                    Err(error) => {
                        eprintln!("Ошибка приёма UDP-датаграммы: {}", error);
                        // Обработка в зависимости от ошибки
                        continue;
                    }
                }

                std::thread::sleep(Duration::from_millis(500));
            }

            println!("Завершение UDP-receiver'а ...");
    });

    return udp_receiver_join_handle;
}

// Запуск UDP-пингера
fn launch_udp_ping(udp_socket: UdpSocket, ping_to_addr: SocketAddr, shutdown_flag: Arc<AtomicBool>) -> JoinHandle<()> {
    // Поток UDP-пинг 
    let udp_ping_join_handle = std::thread::spawn(move || {
        // Сообщение PING, с newline
        let ping_message = b"PING\n";
        
        while !shutdown_flag.load(Ordering::SeqCst) {
            // Отправляем PING на сервер
            match udp_socket.send_to(ping_message, ping_to_addr) {
                Ok(bytes_sent) => {
                println!("PING отправлен ({} байт) на {}", bytes_sent, ping_to_addr);
                }
                Err(e) => {
                    eprintln!("Ошибка отправки PING: {}", e);
                }
            }

            // Тут можно поставить цикл с проверкой флага типа 10 по 200мс, чтобы быстрее среагировать, пока не сделал 
        
            // Ждём 2 секунды перед следующим пингом (как в задании)
            std::thread::sleep(Duration::from_secs(PING_INTERVAL_SECS));
        }
    
        println!("Завершение PING-sender'а ...");
    });

    return udp_ping_join_handle;
}

