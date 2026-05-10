
use std::time::{Duration, Instant};

use tokio::{
    sync::mpsc::{Receiver, Sender, channel, error::SendError},
    time::timeout,
};

struct TaskPool<T> {
    tasks: Receiver<T>,
    sender: Sender<T>,
}

impl<T> TaskPool<T> {
    fn new(queue_size: usize) -> Self {
        let (sender, tasks) = channel::<T>(queue_size);
        Self { tasks, sender }
    }

    async fn create(&self, task: T) -> Option<Result<(), SendError<T>>> {
        // sender.send() returns a Future that completes when the message is accepted
        let sender_fututure = self.sender.send(task);
        // Timeout the send operation: if channel is full, wait up to 100ms

        // Это именно tokio::timeout, future здесь поллится
        timeout(Duration::from_millis(100), sender_fututure).await.ok()
    } 

    async fn pull_task(&mut self) -> Option<T> {
        self.tasks.recv().await
    }
}

#[tokio::main]
async fn main() {
    // Bounded-канал, с очередью == 2
    let mut tasks = TaskPool::new(2);

    // Создаём две таски - очередь полная
    assert_eq!(tasks.create(()).await, Some(Ok(())));  // ✅ Sent
    assert_eq!(tasks.create(()).await, Some(Ok(())));  // ✅ Sent (channel now full)

    // Try to send a 3rd item — channel is full, so send() will wait
    let start = Instant::now();
    assert_eq!(tasks.create(()).await, None);  // ⏰ Timeout after ~100ms → returns None
    let end = start.elapsed();

    // Грубая проверка, что задержка в пределах (90ms; 110ms)
    assert!(end > Duration::from_millis(90));
    assert!(end < Duration::from_millis(110));

    // Одну задачу вытаскиваем
    assert_eq!(tasks.pull_task().await, Some(()));
    
    // И новая может залезть
    assert_eq!(tasks.create(()).await, Some(Ok(())));
}

/*** Полезное описание от "джуна" ***/

// Time 0ms:   Runtime starts main() task
//             ├─ create().await #1 → channel empty → send completes immediately
//             ├─ create().await #2 → channel has 1 slot → send completes immediately
//             ├─ create().await #3 → channel FULL → send() returns Pending
//             │  ├─ timeout(100ms) timer starts
//             │  └─ main() task YIELDS (Pending)
//             │
// Time 0–100ms: [main() task is parked]
//               [Runtime has NO OTHER TASKS to run]
//               [Thread is idle, but not blocked!]
//             │
// Time ~100ms: Timer expires → timeout() returns Err(Elapsed)
//             ├─ main() task resumes
//             ├─ .ok() converts to None
//             └─ assertion passes
//             │
// Time ~100ms: pull_task().await → channel has items → recv completes immediately
//             │
// Time ~100ms: create().await #4 → channel has space → send completes immediately
//             │
// Time ~100ms: main() completes → runtime shuts down

// • main() yields at .await points ✅
// • But there are NO OTHER TASKS scheduled ❌
// • So the thread sits idle while waiting

// This is still correct async code — it just doesn't demonstrate concurrency
// because there's nothing else to run!

// To see concurrency, you'd need:
// tokio::spawn(async { ... })  // ← Create additional tasks

/*** С демонстрацией, чтобы main не просто простаивал, ***/

// #[tokio::main]
// async fn main() {
//     let mut tasks = TaskPool::new(2);
    
//     // 🔥 Spawn a consumer task that pulls items
//     let consumer = tokio::spawn(async move {
//         while let Some(item) = tasks.pull_task().await {
//             println!("Processed: {:?}", item);
//         }
//     });
    
//     // 🔥 Spawn multiple producer tasks
//     let producers: Vec<_> = (0..5)
//         .map(|i| tokio::spawn(async move {
//             tasks.create(i).await;
//         }))
//         .collect();
    
//     // Wait for all producers
//     for p in producers { p.await.unwrap(); }
    
//     // Drop senders to close channel, then wait for consumer
//     drop(tasks.sender);
//     consumer.await.unwrap();
// }