use tokio::time::{sleep, Duration};

async fn task(name: &str, delay_ms: u64) {
    println!("🚀 {} started", name);
    sleep(Duration::from_millis(delay_ms)).await;
    println!("✅ {} finished", name);
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // Последовательно запускаем, в каждом .await ждём
    println!("=== Sequential .await ===");
    task("A", 300).await;
    task("B", 300).await;
    
    // Параллельно с ожиданием всех (но в одном потоке)
    println!("\n=== join! (concurrent, wait all) ===");
    tokio::join!(task("C", 300), task("D", 300));
    
    // Ждём первую закончившуюся таску
    println!("\n=== select! (concurrent, wait first) ===");
    tokio::select! {
        _ = task("Fast", 100) => println!("Fast won!"),
        _ = task("Slow", 1000) => println!("Slow won!"),
    }

    // Отдельная таска для runtime'а: задача main продолжает работу, не ждёт
    println!("\n=== spawn (background) ===");
    let spawned_task = tokio::spawn(task("Background", 500));
    println!("Main continues immediately!");
    spawned_task.await.ok();
}