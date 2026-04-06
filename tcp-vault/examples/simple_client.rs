use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // Подключаемся и клонируем stream для reader'а
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut reader = BufReader::new(stream.try_clone()?);

    // Читаем приветствие
    for _ in 0..1 {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        print!("{}", line);
    }

    let stdin = io::stdin();
    loop {
        // Показываем промпт
        print!("vault-client > ");
        io::stdout().flush()?;

        // Вводим команду
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            continue;
        }

        // Отправляем команду
        stream.write_all(trimmed_input.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;

        // Читаем ответ - ОДНУ строку (без внутреннего цикла!)
        let mut reply_buffer = String::new();
        let read_bytes_cnt = reader.read_line(&mut reply_buffer)?;

        if read_bytes_cnt == 0 {
            println!("Server closed connection");
            return Ok(());
        }

        print!("{}", reply_buffer);

        if trimmed_input.eq_ignore_ascii_case("EXIT") {
            break;
        }
    }
    Ok(())
} 