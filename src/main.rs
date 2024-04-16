#![warn(clippy::pedantic, clippy::nursery, clippy::perf, clippy::style)]
#![deny(
    clippy::suspicious,
    clippy::correctness,
    clippy::complexity,
    clippy::missing_const_for_fn,
)]
#![forbid(unsafe_code)]
#![allow(clippy::must_use_candidate)]

fn main() {
    println!("Hello, world!");
}
