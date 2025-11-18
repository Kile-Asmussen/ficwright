#![feature(format_args_nl)]
#![feature(associated_type_defaults)]
#![allow(unused)]

use clap::Parser;
use rootcause::Report;

use crate::command::Ficwright;

pub mod command;
pub mod config;
pub mod driver;

pub mod forms;
pub mod utils;

type Result<X> = std::result::Result<X, Report>;

#[tokio::main]
async fn main() -> Result<()> {
    tokio::process::Command::new("pkill")
        .arg("geckodriver")
        .output()
        .await?;

    let res = Ficwright::parse().run().await;

    res
}
