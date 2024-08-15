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

use anyhow::Context;
use clap::{Parser, Subcommand};
use tokio::process::Command;

use irlman::{get_manual, upload_manual, Manual};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Upload {
        company: String,
        product: String,
        path: PathBuf,
    },

    Get {
        company: String,
        product: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        SubCommands::Upload {
            company,
            product,
            path,
        } => {
            let manual = Manual { company, product };

            upload_manual(manual, path).await.context("Failed to upload manual")?;
        }
        SubCommands::Get { company, product } => {
            let manual = Manual { company, product };

            let mut path = PathBuf::from("/tmp/");

            path.push(manual.to_path());

            let file = get_manual(manual).await.context("Failed to get the manual")?;

            tokio::fs::write(path.clone(), file).await.unwrap();

            println!("opening file");

            Command::new("xdg-open")
                .arg(path)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("failed to open manual");
        }
    }

    Ok(())
}
