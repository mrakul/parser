
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use tokio::time::Sleep;

#[derive(Debug)]
enum TimeoutFuture<O> {
    Result(O),
    Timeout,
}

fn timeouted_read<F: IntoFuture>(
    timeout: Duration,
    future: F,
) -> TimeoutedFuture<<F as IntoFuture>::IntoFuture> {
    let future = future.into_future();
    let sleep = tokio::time::sleep(timeout);
    TimeoutedFuture { future, sleep }
}

struct TimeoutedFuture<F: Future> {
    future: F,
    sleep: Sleep,
}

impl<F: Future> TimeoutedFuture<F> {
    fn future(self: Pin<&mut Self>) -> Pin<&mut F> {
        unsafe { self.map_unchecked_mut(|s| &mut s.future) }
    }

    fn sleep(self: Pin<&mut Self>) -> Pin<&mut Sleep> {
        unsafe { self.map_unchecked_mut(|s| &mut s.sleep) }
    }
}

impl<F: Future> Future for TimeoutedFuture<F> {
    type Output = TimeoutFuture<F::Output>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let future = self.as_mut().future();

        match future.poll(cx) {
            Poll::Ready(output) => return Poll::Ready(TimeoutFuture::Result(output)),
            Poll::Pending => {}
        }

        let sleep = self.as_mut().sleep();

        match sleep.poll(cx) {
            Poll::Ready(()) => Poll::Ready(TimeoutFuture::Timeout),
            Poll::Pending => Poll::Pending,
        }
    }
} 

// Тесты
#[tokio::main]
async fn main() {
    let instant = async { 0 };

    let result = timeouted_read(Duration::from_millis(123), instant).await;
    
    println!("Result: {result:?}"); // Result(0)

    let wait100 = async {
        let delay = 100;
        tokio::time::sleep(Duration::from_millis(delay)).await;
        delay
    };

    let result = timeouted_read(Duration::from_millis(123), wait100).await;
    println!("Result: {result:?}"); // Result(100)

    let wait150 = async {
        let delay = 150;
        tokio::time::sleep(Duration::from_millis(delay)).await;
        delay
    };

    let result = timeouted_read(Duration::from_millis(123), wait150).await;
    println!("Result: {result:?}"); // Timeout

    let never = std::future::pending::<usize>();
    let result = timeouted_read(Duration::from_millis(123), never).await;
    println!("Result: {result:?}"); // Timeout
}