// src/main.rs

use std::net::TcpListener;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

mod server;
mod vault;

use crate::server::handle_client;
use crate::vault::Vault;

fn main() -> std::io::Result<()> {
    
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    // Новое хранилище, обёрнутое в мьютекс и Arc
    let vault = Arc::new(Mutex::new(Vault::new(10))); // лимит 10 ячеек

    // Фактически бесконечный цикл, при возникновении соединения создаёт новый сервер
    // (блокирующий вызов .incoming(), аналог accept() в цикле)
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Здесь: main создаёт соединения с обработкой клиента, то есть каждый клиент обрабатывается
                // в своём потоке, фактически каждое соединение
                // Но при этом vault - разделяемое между потоками через Arc::clone(&vault)
                let vault = Arc::clone(&vault);
                thread::spawn(move || {
                    handle_client(stream, vault);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    // Подключение классическое:
    // > nc 127.0.0.1 7878

    Ok(())
} 