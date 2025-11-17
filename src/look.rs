use clap::Parser;
use thirtyfour::WebDriver;

use crate::{command::*, driver::DriverExts, utils::*, *};

#[derive(Debug, Clone, Copy, Parser)]
pub struct Ao3Look;

impl WebRunnable for Ao3Look {
    async fn run(self, driver: &mut WebDriver, opt: Ao3Opts) -> Result<()> {
        if let Ok(cookies) = opt.get_cookies().await {
            driver.add_cookies(&cookies).await?;
        }

        prompt("Waiting for user input...").await?;

        Ok(())
    }
}
