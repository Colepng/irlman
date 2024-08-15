#![warn(clippy::pedantic, clippy::nursery, clippy::perf, clippy::style)]
#![deny(
    clippy::suspicious,
    clippy::correctness,
    clippy::complexity,
    clippy::missing_const_for_fn
)]
#![forbid(unsafe_code)]
#![allow(clippy::must_use_candidate)]

use std::path::PathBuf;

use lazy_static::lazy_static;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Manual {
    pub company: String,
    pub product: String,
}

impl Manual {
    pub fn to_path(&self) -> PathBuf {
        let path = format!("{}-{}.pdf", self.company, self.product);
        PathBuf::from(path)
    }
}

lazy_static! {
    static ref CLIENT: Client = Client::new();
}


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Path must exist to read file")]
    PathDoesNotEixist(#[from] std::io::Error),
    #[error("Failed to send upload request, is ip address correct and the server online?")]
    FailedToSendRequest(#[from] reqwest::Error),
}

pub async fn get_manual(manual: Manual) -> anyhow::Result<Vec<u8>> {
    let respone = CLIENT
        .get(format!(
            "http://127.0.0.1:3000/get/{}/{}",
            manual.company, manual.product
        ))
        .send()
        .await?;

    let data = respone.bytes().await?;

    Ok(data.to_vec())
}

pub async fn upload_manual(manual: Manual, path: PathBuf) -> Result<(), Error>{
    use reqwest::multipart::Form;
    use reqwest::multipart::Part;

    let file = tokio::fs::read(path).await?;
    let file_len = file.len() as u64;

    let part = Part::stream_with_length(file, file_len).file_name("test.txt");

    let form = Form::new().part("manual", part);

    CLIENT
        .post(format!(
            "http://127.0.0.1:3000/upload/{}/{}",
            manual.company, manual.product
        ))
        .multipart(form)
        .send()
        .await?;

    Ok(())
}
