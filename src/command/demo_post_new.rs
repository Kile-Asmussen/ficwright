use std::{path::PathBuf, time::Duration};

use clap::Parser;
use thirtyfour::By;

use crate::{
    command::{Ao3Opts, WebRunnable},
    config::Fanfiction,
    driver::DriverExts,
    forms::work_form::WorkForm,
    utils::prompt,
    *,
};

#[derive(Debug, Clone, Parser)]
pub struct Ao3DemoPostNew {}

impl WebRunnable for Ao3DemoPostNew {
    async fn run(self, driver: &mut thirtyfour::WebDriver, opt: Ao3Opts) -> Result<()> {
        driver.add_cookies(&opt.get_cookies().await?).await?;

        driver.ao3("/works/new").await?;

        let work_form = WorkForm::from(driver.find(By::Id("work-form")).await?);

        work_form.tags.resolve().await?.demonstrate().await?;

        tokio::time::sleep(Duration::from_millis(500)).await;

        Ok(())
    }
}
