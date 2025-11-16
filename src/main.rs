#![feature(format_args_nl)]

use clap::Parser;
use rootcause::Report;
use std::{process::Stdio, time::Duration};
use thirtyfour::{DesiredCapabilities, WebDriver};

use crate::command::Ficwright;

pub mod command;
pub mod config;
pub mod login;
pub mod logout;
pub mod look;
pub mod utils;

type Result<X> = std::result::Result<X, Report>;

pub const AO3: &'static str = "https://archiveofourown.org";

#[tokio::main]
async fn main() -> Result<()> {
    let command = Ficwright::parse();

    tokio::process::Command::new("pkill")
        .arg("geckodriver")
        .output()
        .await?;

    let (out, err) = command.gecko_logfile().await?;
    let mut child = tokio::process::Command::new("geckodriver")
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err)
        .spawn()?;

    tokio::time::sleep(Duration::from_millis(1000)).await;

    let mut driver =
        WebDriver::new("http://localhost:4444", DesiredCapabilities::firefox()).await?;

    command.run(&mut driver).await?;

    driver.quit().await?;
    child.kill().await?;
    child.wait().await?;

    Ok(())
}
