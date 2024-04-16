#![warn(clippy::pedantic, clippy::nursery, clippy::perf, clippy::style)]
#![deny(
    clippy::suspicious,
    clippy::correctness,
    clippy::complexity,
    clippy::missing_const_for_fn,
)]
#![forbid(unsafe_code)]
#![allow(clippy::must_use_candidate)]

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // Name of manual
    name: String,
}

fn main() {
    let args = Args::parse();

    println!("You are trying to access, {}", args.name);
}
