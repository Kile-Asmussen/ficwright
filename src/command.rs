use std::{
    os::fd::{AsRawFd, FromRawFd},
    path::PathBuf,
    process::Stdio,
    time::Duration,
};

use crate::{
    Result,
    config::CookieConfig,
    driver::DriverExts,
    login::Ao3Login,
    logout::Ao3Logout,
    look::Ao3Look,
    post_new::Ao3PostNew,
    template::{DebugTemplateCommand, TemplateCommand},
};
use clap::{Parser, Subcommand};
use thirtyfour::prelude::*;
use tokio::process::Child;

#[derive(Debug, Parser)]
pub struct Ficwright {
    #[clap(flatten)]
    opts: FicwrightOpts,
    #[clap(subcommand)]
    command: Command,
}

impl Ficwright {
    pub async fn run(self) -> Result<()> {
        self.command.run(self.opts).await
    }
}

#[derive(Debug, Clone, Default, Parser)]
pub struct FicwrightOpts {
    #[clap(long)]
    pub gecko_logfile: Option<PathBuf>,
}

impl FicwrightOpts {
    async fn gecko_logfile(&self) -> Result<(Stdio, Stdio)> {
        if let Some(f) = &self.gecko_logfile {
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

#[derive(Debug, Clone, Subcommand)]
enum Command {
    Ao3(Ao3Command),
    #[clap(flatten)]
    Local(LocalCommand),
}

impl Command {
    pub async fn run(self, fw_opts: FicwrightOpts) -> Result<()> {
        Ok(match self {
            Self::Ao3(mut ao3_command) => {
                ao3_command.options.general_options = fw_opts;
                ao3_command.command.pre(&ao3_command.options).await?;

                let mut gecko = start_gecko(&ao3_command.options.general_options).await?;

                let res = ao3_command.run().await;

                gecko.kill().await?;
                gecko.wait().await?;

                res?;
            }
            Self::Local(local_command) => local_command.run(fw_opts).await?,
        })
    }
}

async fn start_gecko(opts: &FicwrightOpts) -> Result<Child> {
    let (out, err) = opts.gecko_logfile().await?;
    let child = tokio::process::Command::new("geckodriver")
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err)
        .spawn()?;

    tokio::time::sleep(Duration::from_millis(500)).await;

    Ok(child)
}

#[derive(Debug, Clone, Subcommand)]
enum LocalCommand {
    Template(TemplateCommand),
    DebugTemplate(DebugTemplateCommand),
}

impl Runnable for LocalCommand {
    async fn run(self, opts: FicwrightOpts) -> Result<()> {
        Ok(match self {
            Self::Template(template) => template.run(opts).await?,
            Self::DebugTemplate(debug) => debug.run(opts).await?,
        })
    }
}

pub trait Runnable {
    fn run(self, opts: FicwrightOpts) -> impl Future<Output = Result<()>>;
}

#[derive(Debug, Clone, Parser)]
struct Ao3Command {
    #[clap(flatten)]
    options: Ao3Opts,
    #[clap(subcommand)]
    command: Ao3Script,
}

impl Ao3Command {
    pub async fn run(self) -> Result<()> {
        let mut capabilities = DesiredCapabilities::firefox();
        let mut driver = WebDriver::new("http://localhost:4444", capabilities).await?;

        driver.ao3("").await?;

        self.command.run(&mut driver, self.options).await?;

        driver.quit().await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Subcommand)]
enum Ao3Script {
    Login(Ao3Login),
    Logout(Ao3Logout),
    Look(Ao3Look),
    PostNew(Ao3PostNew),
}

impl WebRunnable for Ao3Script {
    async fn pre(&mut self, opts: &Ao3Opts) -> Result<()> {
        match self {
            Self::Login(ao3_login) => ao3_login.pre(opts).await,
            Self::Logout(ao3_logout) => ao3_logout.pre(opts).await,
            Self::Look(ao3_look) => ao3_look.pre(opts).await,
            Self::PostNew(ao3_post_new) => ao3_post_new.pre(opts).await,
        }
    }
    async fn run(self, driver: &mut WebDriver, opt: Ao3Opts) -> Result<()> {
        match self {
            Self::Login(ao3_login) => ao3_login.run(driver, opt).await,
            Self::Logout(ao3_logout) => ao3_logout.run(driver, opt).await,
            Self::Look(ao3_look) => ao3_look.run(driver, opt).await,
            Self::PostNew(ao3_post_new) => ao3_post_new.run(driver, opt).await,
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub struct Ao3Opts {
    #[clap(long, short = 'c')]
    #[clap(default_value = "~/.ao3.cookie")]
    pub cookies: PathBuf,

    #[clap(skip)]
    pub general_options: FicwrightOpts,
}

impl Ao3Opts {
    pub async fn get_cookies(&self) -> Result<CookieConfig> {
        CookieConfig::read_from_file(&self.cookies).await
    }
}

pub trait WebRunnable {
    fn pre(&mut self, opts: &Ao3Opts) -> impl Future<Output = Result<()>> {
        let _ = opts;
        async { Ok(()) }
    }
    fn run(self, driver: &mut WebDriver, opt: Ao3Opts) -> impl Future<Output = Result<()>>;
}
