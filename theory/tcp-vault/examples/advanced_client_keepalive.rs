use socket2::{Domain, Protocol, Socket, Type};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};


fn main() -> io::Result<()> {
   let (stream, reader) = connect()?;

    let stream = Arc::new(Mutex::new(stream));
    let reader = Arc::new(Mutex::new(reader));

    // Keepalive-поток
    {
        // Клонируем 
        let stream_arc = Arc::clone(&stream);
        let reader_arc = Arc::clone(&reader);
        
        // Создаём отдельный поток, которому передаём  
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));

            // Лочим на чтение и на запись
            let mut write_stream_mutex = stream_arc.lock().unwrap();
            let mut read_stream_mutex = reader_arc.lock().unwrap();

            match send_ping(&mut *write_stream_mutex, &mut *read_stream_mutex) {
                Err(e) =>{
                    eprintln!("Keepalive failed. Reconnecting... {:?}", e);
                    let (new_s, new_r) = reconnect();
                    *write_stream_mutex = new_s;
                    *read_stream_mutex = new_r;
                }
                Ok(latency) => println!("PONG (latency: {}ms)", latency),
            }
        });
    }

    // Основной интерактивный цикл
    let stdin = io::stdin();
    
    loop {
        print!("vault> ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let command = input.trim();

        if command.is_empty() {
            continue;
        }
        if command.eq_ignore_ascii_case("EXIT") {
            println!("Bye!");
            break;
        }

        // Лочим на чтение и на запись аналогично, пока не получим ответ или не сделаем reconnect
        let mut stream = stream.lock().unwrap();
        let mut reader = reader.lock().unwrap();

        match send_command(&mut *stream, &mut *reader, command) {
            Ok(resp) => print!("{}", resp),
            Err(e) => {
                eprintln!("Command failed: {}. Reconnecting...", e);
                let (new_s, new_r) = reconnect();
                *stream = new_s;
                *reader = new_r;
            }
        }
    }

    Ok(())
}

// Подключение к серверу
fn connect() -> io::Result<(TcpStream, BufReader<TcpStream>)> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

    // Настраиваем сокет и подключаемся
    socket.set_keepalive(true)?;
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        socket.set_tcp_keepalive(
            &socket2::TcpKeepalive::new()
                .with_time(Duration::from_secs(10))
                .with_interval(Duration::from_secs(5)),
        )?;
    }

    let addr: SocketAddr = "127.0.0.1:7878".parse().unwrap();
    socket.connect(&addr.into())?;

    let stream: TcpStream = socket.into();
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    // Клонируем reader'а
    let mut reader = BufReader::new(stream.try_clone()?);

    // Читаем welcome message один раз
    let mut line = String::new();
    reader.read_line(&mut line)?;
    print!("{}", line);

    println!("Connected to server!");
    Ok((stream, reader))
}

// Реконнект
fn reconnect() -> (TcpStream, BufReader<TcpStream>) {
    
    loop {
        match connect() {
            Ok(pair) => return pair,
            Err(e) => {
                eprintln!("Reconnect failed: {}. Retrying in 2s...", e);
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

// Отправка команды
fn send_command(
    stream: &mut TcpStream,
    reader: &mut BufReader<TcpStream>,
    command: &str,
) -> io::Result<String> {
    stream.write_all(command.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    let mut buffer = String::new();

    // Читаем в строку
    let bytes_read_cnt = reader.read_line(&mut buffer)?;
    
    if bytes_read_cnt == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Server closed connection",
        ));
    }
    Ok(buffer)
}

// Отправка PING
fn send_ping(stream: &mut TcpStream, reader: &mut BufReader<TcpStream>) -> io::Result<u64> {
    let start = Instant::now();
    stream.write_all(b"PING\n")?;
    stream.flush()?;

    let mut buffer = String::new();
    let bytes = reader.read_line(&mut buffer)?;
    if bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Server closed connection",
        ));
    }

    if buffer.trim() == "PONG" {
        Ok(start.elapsed().as_millis() as u64)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid response: {}", buffer),
        ))
    }
}
