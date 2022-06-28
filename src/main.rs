use futures::executor::block_on;
use rand::prelude::*;

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

pub struct RandomFuture {}

impl SimpleFuture for RandomFuture {
    type Output = f64;
    fn poll(&mut self, _wake: fn()) -> Poll<Self::Output> {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        if y > 0.9 {
            Poll::Ready(y)
        } else {
            // probably need to utilize wake() here
            Poll::Pending
        }
    }
}

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world();
    block_on(future);
}
