use tokio::task;
use std::thread;

async fn print_thread_id(name: &str) {
    println!("{} running on thread {:?}", name, thread::current().id());
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    println!("=== join! (same task) ===");
    // Понятно - это просто функкция, которая поллит, что все задачи закончились (но в одном потоке)
    tokio::join!(
        print_thread_id("A"),
        print_thread_id("B"),
        print_thread_id("C"),
    );
    
    println!("\n=== spawn (independent tasks) ===");
    let handles: Vec<_> = (0..3)
        .map(|i| tokio::spawn(async move {
            print_thread_id(&format!("Spawned {}", i)).await;
        }))
        .collect();
    for h in handles { h.await.ok(); }
}