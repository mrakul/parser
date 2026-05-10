use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Подключение к http://google.com
    let mut stream = TcpStream::connect(("google.com", 80)).await?;

    // Отправка GET-запроса
    // write_all аналогичен синхронному write_all, только возвращает фьючерс
    stream.write_all(b"GET / HTTP/1.1\r\nHost: google.com\r\n\r\n").await?;

    // Упрощено для примера, но обычно так не делают
    // Работает для этого примера, так как
    // гугл отсылает ответ одним tcp-пакетом
    let mut buf = [0u8; 1024];
    let len = stream.read(&mut buf).await?;
    let buf = buf[..len].to_vec();
    let output = String::from_utf8(buf).unwrap();

    println!("Returned:\n{output}");
    Ok(())
}