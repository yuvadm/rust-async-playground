# Rust async playground

## Notes

Research started in 2019 - https://users.rust-lang.org/t/designing-an-async-flow-of-data/29148 and got pointed to `Stream` and `Sink` which are the appropriate abstractions for async *iteration*.

They weren't stable at the time, but seem to be maturing in the `futures` crate:

https://docs.rs/futures/0.3.6/futures/sink/trait.Sink.html

https://docs.rs/futures/0.3.6/futures/stream/trait.Stream.html

This is the relevant chapter in the Rust Async Book - https://rust-lang.github.io/async-book/05_streams/02_iteration_and_concurrency.html

Meanwhile it seems some features aren't fully stable in Rust just yet. Some crates provide relevant functionality

https://github.com/taiki-e/futures-async-stream

https://github.com/tokio-rs/async-stream

This page has a good review https://blog.yoshuawuyts.com/rust-streams/

More good stuff at https://www.qovery.com/blog/a-guided-tour-of-streams-in-rust

## Design

Each block processes input and out asynchronously.
