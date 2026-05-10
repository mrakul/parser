// Важно: здесь контекст создаётся tokio, цикл поллинга делается tokio, нужно только отдать waker и вызвать waker.wake(),
// остальное сделает tokio

use std::time::Duration;
use std::task::Context;

#[tokio::main]
async fn main() {
    // Context создаётся tokio

    println!("Before wait");
    
    // Это вызов poll(), который пока всегда возвращает Pending
    // В последней реализации .await означает "полить этот фьючерс пока не Ready"
    wait_for(std::time::Duration::from_secs(2)).await;

    println!("After wait");
} 

// Из-за того, что возвращается фьючерс, это функция является асинхронной. 
// Поэтому можно вызывать на ней .await!, который является просто синтаксическим сахаром для поллинга.
// fn wait_for(duration: Duration) -> impl Future<Output = ()> {
    // Возвращает WaitFor?

    // Returns a WaitFor struct — which IS the Future
    // WaitFor { duration }
// }

// struct WaitFor {
//     duration: Duration,
// }

struct WaitFor {
    duration: Duration,
    waited: bool,
}

// Теперь возвращаем не Фьючерс?
// Returns WaitFor directly (which IS a Future — see impl below)
fn wait_for(duration: Duration) -> WaitFor {
    WaitFor { duration, waited: false }
}

// Реализация Future именно для WaitFor
impl Future for WaitFor {
    type Output = ();

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> 
    {    
        // 1️⃣ CHECK STATE: Have we already started waiting?
        if self.waited {
            // ✅ Yes → we're done! Return Ready
            return std::task::Poll::Ready(());
        }

        // 2️⃣ FIRST POLL: Mark that we've started
        self.waited = true;

        // 3️⃣ REGISTER WAKER: Clone it so background thread can wake us
        let waker = cx.waker().clone();
        let duration = self.duration;

        // 4️⃣ SPAWN BACKGROUND THREAD: Sleep then notify
        std::thread::spawn(move || {
            std::thread::sleep(duration);  // Block this thread for `duration`
            waker.wake();                       // 🔥 Notify runtime: "I'm ready!"
        });

        // 5️⃣ RETURN PENDING: "Not done yet, but I've arranged to be woken"
        std::task::Poll::Pending
    }
}