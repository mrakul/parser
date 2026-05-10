use tokio::{task_local, time::sleep};
use std::time::Duration;

async fn task1() {
    sleep(Duration::from_millis(500)).await;
    println!("task1");
}

async fn task2() {
    sleep(Duration::from_millis(500)).await;
    println!("task2");
} 

// #[tokio::main]
// 1. Указать, что многопоточно:
#[tokio::main(flavor = "multi_thread")]
// 2. С указанием количества потоков: 
// #[tokio::main(flavor = "multi_thread", worker_threads = 8)]
// 3. Однопоточность:
// #[tokio::main(flavor = "current_thread")]
async fn main() {

    // (!) Код ниже выполняется в одном Runtime

    // 0. Синхронные вызовы - Runtime ждёт каждого await последовательно
    let res1 = task1().await;
    let res2 = task2().await;
    
    // 1. Запуск параллельно и ожидание их всех
    // tokio::join!(task1(), task2());
    let (res1, res2) = tokio::join!(
        task1(),
        task2(),
    );

    println!("join! => both tasks finished");

    // 2. Ожидание одного из с выполнением логики: с одинаковой задержкой выведется или task1, или task2
    // При этом выполнение второго отменится
    let res = tokio::select! {
        // res1 = task1() => {
        //     // возможность выполнения логики
        //     res1
        // },
        res1 = task1() => res1,
        res2 = task2() => res2,
    };


    // 3. Функция spawn добавит задачу в очередь задач tokio и при свободном исполнителе запустит задачу.
    // А если текущий рантайм tokio многопоточный, то это позволит использовать многопоточность, так как задачи, созданные через spawn, tokio может распределять между потоками. 
    // Также возвращает JoinHandle, аналогичные по функционалу JoinHandle из стандартной библиотеки.
    // Но функцию невозможно выполнить из других рантаймов.

    let handle = tokio::spawn(async {
        println!("spawned");
    });
    // main может напечататься и до spawned, и после
    // то есть примерно такое же поведение, как и у многопоточности
    println!("Placed after spawned");
    // Дождаться завершения можно, вызвав .await,
    // (как .join() в std::thread::JoinHandle)
    handle.await.unwrap();

    // Если вызываешь без .await, только создаётся executor
    sleep(Duration::from_millis(1500)).await;

}

// А ещё в библиотеке futures есть функция join_all, которая делает то же самое, но на вход она принимает итератор по фьючерсам:

// use futures::future::join_all;
// async fn foo(i: u32) -> u32 { i }

// async fn futures_vec_fn() {
//     let futures_vec = vec![foo(1), foo(2), foo(3)];   
//     assert_eq!(join_all(futures_vec).await, [1, 2, 3]); 
// }


// Вот это: 
// #[tokio::main]
// async fn main() {
//     do_something().await;
// } 

// Аналог этого (и другие случаи с runtime'ом разворачиваются сюда)
// fn main() {
//     tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(async {
//             do_something().await;
//         })
// } 


// (!) При этом выполнение второго фьючерса отменится. Про то, какие фьючерсы не стоит отменять, будет рассказано в следующем уроке. К примеру, с помощью select можно отменять ожидание UDP пакета с тайм-аутом:

// use tokio::net::UdpSocket;
// use tokio::time::sleep;
// use std::io;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let sock = UdpSocket::bind("0.0.0.0:8080").await?;
//     let mut buf = [0; 1024];
//     loop {
//         tokio::select! {
//             data = sock.recv_from(&mut buf) => {
//                 let (len, addr) = data?;
//                 println!("{:?} bytes received from {:?}", len, addr);
//             }
//             _ = sleep(Duration::from_secs(10)) => {}
//         }
//     }
// } 