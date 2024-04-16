#![warn(clippy::pedantic, clippy::nursery, clippy::perf, clippy::style)]
#![deny(
    clippy::suspicious,
    clippy::correctness,
    clippy::complexity,
    clippy::missing_const_for_fn,
)]
#![forbid(unsafe_code)]
#![allow(clippy::must_use_candidate)]

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World"} ));

    let listerner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listerner, app).await.unwrap();
}
