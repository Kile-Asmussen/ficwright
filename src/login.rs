use crate::{command::*, config::*, driver::DriverExts, utils::*, *};
use clap::Parser;
use thirtyfour::prelude::*;

#[derive(Debug, Clone, Parser)]
pub struct Ao3Login {}

impl WebRunnable for Ao3Login {
    async fn run(self, driver: &mut WebDriver, opt: Ao3Opts) -> Result<()> {
        if let Ok(cookies) = opt.get_cookies().await {
            driver.add_cookies(&cookies).await?;

            let yn = prompt("Are you logged in? [Y/n]").await?;

            if yn.contains('n') || yn.contains('N') {
                println_async!(
                    "Deleting unusable cookie file: {}",
                    opt.cookies.to_string_lossy()
                )
                .await;
                tokio::fs::remove_file(&opt.cookies).await?;
            }

            return Ok(());
        } else {
            driver.find(By::Id("login-dropdown")).await?.click().await?;
            driver
                .find(By::Id("user_session_login_small"))
                .await?
                .focus()
                .await?;

            prompt("Please log in... [Enter to continue]").await?;
        }

        CookieConfig::new(driver.get_all_cookies().await?)
            .save_to_file(&opt.cookies)
            .await?;

        println_async!("Cookies saved to {}", opt.cookies.to_string_lossy()).await;

        Ok(())
    }
}
