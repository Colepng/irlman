#![warn(clippy::pedantic, clippy::nursery, clippy::perf, clippy::style)]
#![deny(
    clippy::suspicious,
    clippy::correctness,
    clippy::complexity,
    clippy::missing_const_for_fn
)]
#![forbid(unsafe_code)]
#![allow(clippy::must_use_candidate)]

use std::{fs, path::PathBuf, sync::OnceLock};

use axum::{
    body::Body,
    extract::{Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use irlman::Manual;
use serde::Deserialize;
use tokio_util::io::ReaderStream;

static PDF_FOLDER: OnceLock<std::path::PathBuf> = OnceLock::new();

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: u16,
    path: PathBuf,
}

#[tokio::main]
async fn main() {
    let file_contents = fs::read("server.toml").expect("Failed to read config file");
    let file_str = std::str::from_utf8(&file_contents).unwrap();
    let config: Config = toml::from_str(file_str).unwrap();

    PDF_FOLDER.get_or_init(|| config.path);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World" }))
        .route("/get/:company/:product", get(get_manual))
        .route("/upload/:company/:product", post(upload));

    let listerner = tokio::net::TcpListener::bind(format!("{0}:{1}", config.ip, config.port))
        .await
        .unwrap();
    axum::serve(listerner, app).await.unwrap();
}

async fn upload(Path(manual): Path<Manual>, mut multipart: Multipart) {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap().to_vec();

        use tokio::fs;

        let file_name = manual.to_path();
        let path = PDF_FOLDER.get().clone().unwrap();

        let mut path = path.to_path_buf();

        path.push(file_name);

        fs::write(path, data).await.unwrap();
    } else {
        panic!("you no upload file >:( fuck u");
    }
}

async fn get_manual(Path(manual): Path<Manual>) -> impl IntoResponse {
    let file_name = manual.to_path();

    let path = PDF_FOLDER.get().clone().unwrap();

    let mut path = path.to_path_buf();

    path.push(file_name);

    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let stream = ReaderStream::new(file);

    let body = Body::from_stream(stream);

    Ok(body)
}
