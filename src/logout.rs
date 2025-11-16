use clap::Parser;
use rootcause::bail;

use crate::{command::*, utils::*, *};

#[derive(Debug, Clone, Parser)]
pub struct Ao3Logout {
    #[clap(short = 'f')]
    pub force: bool,
}

impl Runnable for Ao3Logout {
    async fn run(&self, driver: &mut WebDriver, opt: &FicwrightOpts) -> Result<()> {
        let Some(cookies) = opt.user().await? else {
            bail!("No cookie file found: {}", opt.cookies.to_string_lossy());
        };

        driver.goto(AO3).await?;

        for cookie in cookies.iter() {
            driver.add_cookie(cookie).await?;
        }

        driver.refresh().await?;

        let mut delete = self.force;

        if !delete {
            let s = prompt("Delete cookie file? [y/N]").await?;
            delete = s.contains("y") || s.contains("Y")
        }

        if delete {
            tokio::fs::remove_file(&opt.cookies).await?;
        }

        Ok(())
    }
}
