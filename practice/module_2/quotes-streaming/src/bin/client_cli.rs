// Для проверки через CLI, отдельный поток UDP от сервера пока можно проверять через nc, отдельно посылая команды:
// Клиент: 
//      nc 127.0.0.1 11000
//      STREAM udp://127.0.0.1:9100 UNH,CRM
//      STREAM udp://127.0.0.1:9200 AAPL,CRM,TEST
// В другом процессе:
//      nc -u -l 9100

// Здесь без пинга, всё в основном потоке

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use quotes_streaming::SERVER_ADDR;

fn main() -> io::Result<()> {
    // Подключаемся и клонируем stream для reader'а
    let to_server_stream = TcpStream::connect(SERVER_ADDR)?;
    let mut to_client_stream = BufReader::new(to_server_stream.try_clone()?);

    // Читаем приветствие
    let mut buffer = [0; 1024];
    let read_count = to_client_stream.read(&mut buffer)?;
    let starting_string = String::from_utf8_lossy(&buffer[..read_count]);
    print!("{}", starting_string);

    let stdin = io::stdin();
    
    // Когда подключились, тут бесконечный цикл
    loop {
        print!("> ");

        if let Err(e) = io::stdout().flush() {
            eprintln!("Warning: проблема вывода в stdout: {}", e);
            // Пока ворнинг - может быть некритично для основной логики
        }

        // Читаем команду из консоли в строку
        let mut input_line = String::new();
        stdin.read_line(&mut input_line)?; 
        
        let trimmed_input = input_line.trim();

        // любые другие команды
        match send_command(&to_server_stream, &mut to_client_stream, trimmed_input) {
            Ok(response) => print!("{}", response),
            Err(e) => {
                // TODO: 
                println!("ERROR: Сервер закрыл соединение ({})", e);
            }
        }
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
                                 "Cервер закрыл соединение",
        ));
    }

    Ok(read_buffer)
}
