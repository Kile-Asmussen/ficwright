use std::path::PathBuf;

use crate::{command::*, config::*, utils::*, *};
use clap::Parser;
use thirtyfour::By;

#[derive(Debug, Clone, Parser)]
pub struct Ao3Login {
    #[clap(long, short = 'o', default_value = ".cookies")]
    pub output: PathBuf,
}

impl Runnable for Ao3Login {
    async fn run(&self, driver: &mut WebDriver, opt: &FicwrightOpts) -> Result<()> {
        driver.goto(AO3).await?;

        if let Some(user) = opt.user().await? {
            for cookie in user.iter() {
                driver.add_cookie(cookie).await?;
            }
            driver.refresh().await?;

            let yn = prompt("Are you logged in? [Y/n]").await?;

            if yn.contains('n') || yn.contains('N') {
                println_async!("Deleting {}", opt.cookies.to_string_lossy()).await;
                tokio::fs::remove_file(&opt.cookies).await?;
            }
        } else {
            driver.find(By::Id("login-dropdown")).await?.click().await?;
            driver
                .find(By::Name("user[remember_me]"))
                .await?
                .click()
                .await?;
            driver
                .find(By::Id("user_session_login_small"))
                .await?
                .focus()
                .await?;

            prompt("Please log in... [Enter to continue]").await?;
        }

        let cookies = driver.get_all_cookies().await?;

        let cookies = cookies.into_iter().map(|c| (c.name, c.value)).collect();

        let cookies = CookieConfig { cookies };
        cookies.save_to_file(&self.output).await?;

        println_async!("Cookies saved to {}", self.output.to_string_lossy()).await;

        Ok(())
    }
}
