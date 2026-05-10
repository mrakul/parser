// async_fetch.rs
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::{Instant, Duration};

// Асинхронно посылаем запрос GET-запрос к хосту и получаем ответ с таймаутами
async fn fetch(host: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect((host, 80)).await?;  // ✅ Yields thread while connecting
    
    // stream.write_all(format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host).as_bytes()).await?;
    // Или: задаём таймаут по выполнению, но здесь слишком быстрая запись в стрим
    if let Err(_) = tokio::time::timeout(Duration::from_millis(1), 
                    stream.write_all(format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host).as_bytes()))
    .await?
    {
        println!("не выполнилось в течение 1 миллисекунды");
    } 
    
    let mut buf = [0u8; 2048];

    // let len = stream.read(&mut buf).await?;  // ✅ Yields thread while waiting for response
    
    // Или так:
    // 🔹 Read with timeout
    let mut buf = [0u8; 2048];
    let len = tokio::time::timeout(
        Duration::from_millis(1),  // Give more time for response
        stream.read(&mut buf)
    )
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "read timeout"))?  // Handle timeout
    ?;  // Handle IO error, extract usize
    
    Ok(String::from_utf8_lossy(&buf[..len]).to_string())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let hosts = ["google.com", "github.com", "rust-lang.org", "tokio.rs", "crates.io"];
    let start = Instant::now();
    
    // 🔥 Launch ALL requests concurrently
    let mut tasks = Vec::new();
    for host in hosts {
        let host = host.to_string();
        // Handle'ы по созданным задачам
        tasks.push(tokio::spawn(async move {
            println!("🔄 Fetching {}...", host);
            let result = fetch(&host).await;
            
            // В tokio есть встроенный тред, который упоминается как worker threads.
            // Он позволяет ожидать результата синхронной функции, можно через await, 
            // не блокируя исполнения, как с ожиданием у обычного thread pool:

            // Здесь создаётся отдельный тред, который ожидает работу синхронной функции, но не заблокирует текущую задачу
            // let res = tokio::task::spawn_blocking(move || {
            //     // cpu-bound задачи. К примеру, перемножение матриц
            //     some_cpu_heavy_wori()
            // }).await?; 
            
            println!("✅ {} done", host);
            (host, result)
        }));
    }
    
    // Wait for all to complete
    for task in tasks {
        let (host, result) = task.await?;
        // Optionally process result here
    }
    
    println!("\n⏱️  Total time: {:?}", start.elapsed());
    Ok(())
}


