use thirtyfour::WebDriver;

use crate::{Result, config::CookieConfig};

pub trait DriverExts {
    fn add_cookies(&self, conf: &CookieConfig) -> impl Future<Output = Result<()>>;

    fn ao3(&self, path: impl AsRef<str>) -> impl Future<Output = Result<()>>;
}

impl DriverExts for WebDriver {
    async fn add_cookies(&self, conf: &CookieConfig) -> Result<()> {
        for cookie in conf.iter() {
            self.add_cookie(cookie).await?;
        }
        self.refresh().await?;
        Ok(())
    }

    async fn ao3(&self, path: impl AsRef<str>) -> Result<()> {
        Ok(self
            .goto(format!("https://archiveofourown.org{}", path.as_ref()))
            .await?)
    }
}
