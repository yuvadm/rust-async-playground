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

async fn hello_world() {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    println!("hello, world! {}", y);
}

fn main() {
    let future = hello_world();
    block_on(future);
}
