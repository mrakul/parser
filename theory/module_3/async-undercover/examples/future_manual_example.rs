// Важно: это пример ручного поллинга, ручного создания контекста, ручного создания фьючерса

use std::{task::{Context, Waker}, time::Duration};
use waker_fn::waker_fn;
use std::thread::sleep;

// Возвращает фьючерс
async fn read_toml() -> String {
    // Асинхронно ждём, когда файл прочитается

    // (!) Важно, что читаем через smol
    let file_context = smol::fs::read_to_string("./Cargo.toml").await.unwrap();

    return file_context;
}

// Пока эту функцию можно воспринимать аналогично этой:
// fn read_toml() -> impl Future<Output = String> {
//     async {
//         smol::fs::read_to_string("./Cargo.toml").await.unwrap()
//     }
// } 


fn main() {
    // Создание контекста с Waker, который при вызове ничего не делает
    let (waker, wait) = make_waker();

    // Context создаём на основе waker'а
    let mut context_waker = Context::from_waker(&waker);

    // Создаём фьючерс
    let future = read_toml();

    // Про Pin будет ниже, пока это рассматривайте как требование для поллинга фьючерсов
    let mut future = std::pin::pin!(future);
    
    // Поллим бесконечно и выходим, когда файл прочитан в строку
    loop {
        // sleep(Duration::from_nanos(50));
        match future.as_mut().poll(&mut context_waker) {
            std::task::Poll::Pending => {
                println!("Pending future");
            }
            std::task::Poll::Ready(value) => {
                println!("Ready! value:\n{value}");
                break;
            }
        }
        wait();
    }
}

/// Returns (waker, wait)
fn make_waker() -> (Waker, impl Fn()) {
    let thread = std::thread::current();

    // When called, unblock the parked thread
    let waker = waker_fn(move || {
        thread.unpark();
    });

    // Block this thread until unpark() is called
    let wait = move || {
        // Go to sleep and wait
        std::thread::park();
    };

    (waker, wait)
}


fn wait_for(duration: Duration) -> impl Future<Output = ()> {
    WaitFor {duration}
}

struct WaitFor {
    duration: Duration,
} 

impl Future for WaitFor {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        std::task::Poll::Pending
    }
}