use bytes::Bytes;

use async_stream::stream;

use futures::stream::Stream;

use futures_util::pin_mut;
// use futures_util::stream::StreamExt;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use tokio::fs::File;
// use std::fs::File;
use tokio::io::BufReader;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;

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
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.i += 1;
        Poll::Ready(Some(self.i))
    }
}

struct RandomSource {
    seed: u32,
    modulus: u32,
    a: u32,
    c: u32,
}

impl Stream for RandomSource {
    type Item = u32;
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.seed = (self.a * self.seed + self.c) % self.modulus;
        Poll::Ready(Some(self.seed))
    }
}

fn double<S: Stream<Item = u32>>(input: S) -> impl Stream<Item = u32> {
    stream! {
        for await val in input {
            yield val * 2
        }
    }
}

fn print_sink<S: Stream<Item = u32>>(input: S) -> impl Stream<Item = ()> {
    stream! {
        for await val in input {
            print!("{}", val);
        }
    }
}

async fn file_source() -> impl Stream<Item = u32> {
    stream! {
        let f = File::open("/home/yuval/dev/rustradio-docs/src/background.md").await.unwrap();
        let s = ReaderStream::new(f);
        loop {
            yield "4".as_bytes()[0] as u32
        }
        // loop {
        //     let val = s.next().await.unwrap();
        //     // yield val
        //     yield Bytes::new()
        // }
    }
}

#[tokio::main]
async fn main() {
    // let number_source = NumberSource { i: 0 };
    // let s = print_sink(double(number_source));
    let s = print_sink(file_source().await);
    pin_mut!(s); // needed for iteration
    s.next().await;
}
