use std::{path::PathBuf, time::Duration};

use clap::Parser;
use thirtyfour::By;

use crate::{
    command::{Ao3Opts, WebRunnable},
    config::Fanfiction,
    driver::DriverExts,
    forms::{
        model::{AgeRating, FicCategory},
        work_form::WorkForm,
    },
    utils::prompt,
    *,
};

#[derive(Debug, Clone, Default, Parser)]
pub struct Ao3PostNew {
    pub fic: PathBuf,

    #[clap(long)]
    pub draft: bool,

    #[clap(skip)]
    pub loaded: Fanfiction,
}

impl WebRunnable for Ao3PostNew {
    async fn pre(&mut self, _opts: &Ao3Opts) -> Result<()> {
        println_async!("Loading fanfic specification {:?}", self.fic);
        self.loaded = Fanfiction::load(&self.fic).await?;
        Ok(())
    }

    async fn run(self, driver: &mut thirtyfour::WebDriver, opt: Ao3Opts) -> Result<()> {
        driver.add_cookies(&opt.get_cookies().await?).await?;

        driver.ao3("/works/new").await?;

        let work_form = WorkForm::from(driver.find(By::Id("work-form")).await?);

        work_form.fill_out(&self.loaded).await?;

        tokio::time::sleep(Duration::from_millis(5000)).await;

        Ok(())
    }
}
