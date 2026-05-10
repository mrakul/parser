use futures::future::join_all;
use std::time::Duration;
use tokio::time::sleep;

// напишите здесь асинхронную функцию handle_connections

async fn handle_connections_my(connection_futures: Vec<impl Future<Output = ()>>) {
    // (!) Операции sleep выполняются параллельно (все 10 запускаются одновременно), но вывод на экран происходит последовательно, 
    // поскольку исполнитель опрашивает фьючерсы в том порядке, в котором они были добавлены в вектор
    
    // Один и тот же polling loop:
    // join_all опрашивает фьючерсы в том порядке, в котором они появляются в итераторе.
    // (?) Поменял время засыпания - 
    join_all(connection_futures).await;
}

// With tokio::spawn:
// - Each future is a separate task with its own scheduling
// - Work-stealing scheduler distributes tasks across threads
// - Thread scheduling + I/O completion order is non-deterministic
// → println! executes in unpredictable order

// With join_all on raw futures:
// - All futures share same polling loop
// - Polled in vector order when ready
// → println! executes in vector order

async fn handle_connections_my_2<I, F>(connection_futures: I)
where
    I: IntoIterator<Item = F>,
    F: Future<Output = ()> + Send + 'static
    // F: Send    => Required by tokio::spawn — tasks may move between worker threads
    // F: 'static => Required by tokio::spawn — tasks must not contain non-static references
{
    let handles: Vec<_> = connection_futures
        .into_iter()
        // Вызываем на каждый элемент - фьючерс, создавая новую таску и собирая хендлеры в коллекцию
        .map(|cur_future| tokio::spawn(cur_future))
        .collect();

    // Здесь каждая задача поллится независимо, но await срабатывает, когда потоки завершены (?)
    join_all(handles).await;
}            

// "Прекрасное" решение от создателей курсов
async fn handle_connections_theirs<I, F>(connections: I)
where
    <F as Future>::Output: Send + 'static,
    I: IntoIterator<Item = F>,
    F: Future + Send + 'static,
{
    let mut handles = Vec::new();
    for connection in connections.into_iter() {
        let handle = tokio::spawn(connection);
        handles.push(handle);
    }
    join_all(handles).await;
} 

// Тесты
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    use std::time::Instant;

    // Создаются фьючерсы
    let connections = {
        let mut connections = Vec::new();

        // 10 штук фьючерсов
        for i in 0..10 {
            // Сразу задаём функцию: выдаёт println!() с номером соединения
            let connection = async move {
                sleep(Duration::from_millis((10 - i) * 100)).await;
                println!("Hello from connection {i}");
            };
            connections.push(connection);
        }
        connections
    };

    let start = Instant::now();
    handle_connections_my(connections).await;
    let end = start.elapsed();

    assert!(end < Duration::from_millis(1500))
}

// use futures::future::join_all;
// async fn foo(i: u32) -> u32 { i }

// async fn futures_vec_fn() {
//     let futures_vec = vec![foo(1), foo(2), foo(3)];   
//     assert_eq!(join_all(futures_vec).await, [1, 2, 3]); 
// }