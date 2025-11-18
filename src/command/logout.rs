use clap::Parser;
use thirtyfour::prelude::*;

use crate::{command::*, driver::DriverExts, *};

#[derive(Debug, Clone, Parser)]
pub struct Ao3Logout {
    #[clap(short = 'k')]
    pub keep: bool,
}

impl WebRunnable for Ao3Logout {
    async fn run(self, driver: &mut WebDriver, opt: Ao3Opts) -> Result<()> {
        driver.add_cookies(&opt.get_cookies().await?).await?;

        driver
            .find(By::Css("a[data-method=\"delete\"]"))
            .await?
            .click()
            .await?;

        if !self.keep {
            tokio::fs::remove_file(&opt.cookies).await?;
        }

        Ok(())
    }
}
