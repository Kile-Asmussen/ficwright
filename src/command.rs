use std::{
    os::fd::{AsRawFd, FromRawFd},
    path::PathBuf,
    process::Stdio,
};

use crate::{Result, config::CookieConfig, login::Ao3Login, logout::Ao3Logout, look::Ao3Look};
use clap::{Parser, Subcommand};
use thirtyfour::prelude::*;

#[derive(Debug, Parser)]
pub struct Ficwright {
    #[clap(flatten)]
    options: FicwrightOpts,
    #[clap(subcommand)]
    command: Ao3Interaction,
}

#[derive(Debug, Clone, Subcommand)]
enum Ao3Interaction {
    Login(Ao3Login),
    Logout(Ao3Logout),
    Look(Ao3Look),
}

impl Runnable for Ao3Interaction {
    async fn run(&self, driver: &mut WebDriver, opt: &FicwrightOpts) -> Result<()> {
        match self {
            Self::Login(ao3_login) => ao3_login.run(driver, opt).await,
            Self::Logout(ao3_logout) => ao3_logout.run(driver, opt).await,
            Self::Look(ao3_look) => ao3_look.run(driver, opt).await,
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub struct FicwrightOpts {
    #[clap(long)]
    pub dry_run: bool,

    #[clap(long)]
    pub gecko_logfile: Option<PathBuf>,

    #[clap(long, short = 'c')]
    #[clap(default_value = ".cookies")]
    pub cookies: PathBuf,

    #[clap(long)]
    pub user: Option<PathBuf>,
}

impl FicwrightOpts {
    pub async fn user(&self) -> Result<Option<CookieConfig>> {
        if tokio::fs::try_exists(&self.cookies).await? {
            Ok(Some(CookieConfig::read_from_file(&self.cookies).await?))
        } else {
            Ok(None)
        }
    }
}

impl Ficwright {
    pub async fn run(&self, driver: &mut WebDriver) -> Result<()> {
        self.command.run(driver, &self.options).await
    }

    pub async fn gecko_logfile(&self) -> Result<(Stdio, Stdio)> {
        if let Some(f) = &self.options.gecko_logfile {
            let f = tokio::fs::File::options()
                .append(true)
                .create(true)
                .open(f)
                .await?
                .into_std()
                .await;

            unsafe {
                Ok((
                    Stdio::from_raw_fd(f.as_raw_fd()),
                    Stdio::from_raw_fd(f.as_raw_fd()),
                ))
            }
        } else {
            Ok((Stdio::null(), Stdio::null()))
        }
    }
}

pub trait Runnable {
    fn run(&self, driver: &mut WebDriver, opt: &FicwrightOpts) -> impl Future<Output = Result<()>>;
}
