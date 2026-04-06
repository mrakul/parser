// Этот код может делать reconnect только в случае посылки команды (read - 3 секунды) и во время посылки проверяет

use socket2::{Domain, Protocol, Socket, Type};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

enum ConnectionResult {
    Exit,
    Lost,
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:7878".parse().unwrap();

    loop {
        // пробуем подключиться
        match connect(&addr) {
            Ok(stream) => {
                println!("Connected to server!");

                match handle_connection(stream) {
                    ConnectionResult::Exit => break,
                    ConnectionResult::Lost => {
                        println!("Connection lost. Reconnecting in 2s...");
                        thread::sleep(Duration::from_secs(2));
                    }
                }
            }
            Err(err) => {
                eprintln!("Connect failed: {}. Retrying in 2s...", err);
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

// socket2 для низкоуровневых вещей
fn connect(addr: &SocketAddr) -> io::Result<TcpStream> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

    // Включаем TCP keepalive
    socket.set_keepalive(true)?;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        socket.set_tcp_keepalive(
            &socket2::TcpKeepalive::new()
                .with_time(Duration::from_secs(10))
                .with_interval(Duration::from_secs(5)),
        )?;
    }

    socket.connect(&addr.clone().into())?;
    let stream: TcpStream = socket.into();

    // тайм-аут на чтение
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;

    Ok(stream)
}

fn handle_connection(stream: TcpStream) -> ConnectionResult {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let stdin = io::stdin();

    // читаем приветствие
    let mut line = String::new();
    if reader.read_line(&mut line).is_ok() {
        print!("{}", line);
    }

    // Когда подключились, тут бесконечный цикл
    loop {
        print!("vault> ");
        io::stdout().flush().unwrap();

        // Читаем команду из консоли в строку
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            return ConnectionResult::Lost;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        // EXIT — выходим из клиента
        if trimmed.eq_ignore_ascii_case("EXIT") {
            println!("Bye!");
            return ConnectionResult::Exit;
        }

        // PING — измеряем задержку
        if trimmed.eq_ignore_ascii_case("PING") {
            match send_ping(&stream, &mut reader) {
                Ok(latency) => println!("PONG (latency: {}ms)", latency),
                Err(e) => {
                    println!("ERROR: server unreachable ({})", e);
                    return ConnectionResult::Lost;
                }
            }
            continue;
        }

        // любые другие команды
        match send_command(&stream, &mut reader, trimmed) {
            Ok(response) => print!("{}", response),
            Err(e) => {
                println!("ERROR: connection lost ({})", e);
                return ConnectionResult::Lost;
            }
        }
    }
}

fn send_ping(mut stream: &TcpStream, reader: &mut BufReader<TcpStream>) -> io::Result<u64> {
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

    let elapsed = start.elapsed().as_millis() as u64;

    if buffer.trim() == "PONG" {
        Ok(elapsed)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid PING response",
        ))
    }
}

fn send_command(
    mut stream: &TcpStream,
    reader: &mut BufReader<TcpStream>,
    command: &str,
) -> io::Result<String> 
{
    stream.write_all(command.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    let mut read_buffer = String::new();
    let bytes_read_cnt = reader.read_line(&mut read_buffer)?;

    if bytes_read_cnt == 0 {
        // сервер закрыл соединение
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof,
                                 "Server closed connection",
        ));
    }

    Ok(read_buffer)
}
