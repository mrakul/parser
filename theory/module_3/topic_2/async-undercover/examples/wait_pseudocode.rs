
fn main() {
    todo!()
}

async fn _wait() -> usize {
    tokio::task::yield_now().await;
    67
}

// При компиляции она превратится в некую машину состояний. 
// Посмотрите на этот псевдокод, который поможет понять, как всё работает на самом деле:

// Это примерный код:

// fn wait() -> Wait {
//     Wait::new()
// }

// enum Wait {
//     Start,
//     PollYieldNow {
//         yield_now: (typeof tokio::task::yield_now()),
//     },
//     Ready
// }

// impl Wait {
//     fn new() -> Self {
//         unsafe { std::mem::zeroed() }
//     }
// }

// impl Future for Wait {
//     type Output = usize;

//     fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
//         use std::task::Poll;

//         loop {
//             match *self.as_mut() {
//                 Self::Start => {
//                     // awaited part of code in function
//                     let yield_now = {
//                         tokio::task::yield_now()
//                     };
//                     *self = Self::PollYieldNow { yield_now }
//                 },
//                 Self::PollYieldNow { ref mut yield_now } => {
//                     match yield_now.poll() {
//                         Poll::Pending => return Poll::Pending
//                         Poll::Ready(_) => {
//                             *self = Self::Ready
//                         }
//                     }
//                 },
//                 Self::Ready => {
//                     return Poll::Ready(67)
//                 }
//                 _ => panic!("wait called after completion")
//             }
//         }
//     }
// } 