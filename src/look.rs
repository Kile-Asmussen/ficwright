use clap::Parser;
use thirtyfour::WebDriver;

use crate::{command::*, utils::*, *};

#[derive(Debug, Clone, Copy, Parser)]
pub struct Ao3Look;

impl Runnable for Ao3Look {
    async fn run(&self, driver: &mut WebDriver, opt: &FicwrightOpts) -> Result<()> {
        driver.goto(AO3).await?;

        if let Some(cookies) = opt.user().await? {
            for cookie in cookies.iter() {
                driver.add_cookie(cookie).await?;
            }
            driver.refresh().await?;
        }

        prompt("Waiting for user input...").await?;

        Ok(())
    }
}
