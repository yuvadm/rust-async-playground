use async_stream::stream;

use futures::stream::Stream;

use futures_util::pin_mut;
use futures_util::stream::StreamExt;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            Poll::Ready("done")
        } else {
            // Ignore this line for now. (this possibly busy waits?)
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn number_source() -> impl Stream<Item = u8> {
    stream! {
        for i in 0..10 {
            yield i
        }
    }
}

struct NumberSource {
    i: u32,
}

impl Stream for NumberSource {
    type Item = u32;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.i += 1;
        Poll::Ready(Some(self.i))
    }
}

fn double<S: Stream<Item = u32>>(input: S) -> impl Stream<Item = u32> {
    stream! {
        for await val in input {
            yield val * 2
        }
    }
}

#[tokio::main]
async fn main() {
    let number_source = NumberSource { i: 0 };
    let s = double(number_source);
    pin_mut!(s); // needed for iteration

    while let Some(value) = s.next().await {
        println!("got {}", value);
    }
}
