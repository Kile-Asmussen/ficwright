use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use thirtyfour::Cookie;

use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieConfig {
    pub cookies: HashMap<String, String>,
}

impl CookieConfig {
    pub async fn save_to_file(&self, file: &Path) -> Result<()> {
        tokio::fs::write(file, toml::to_string_pretty(&self)?).await?;
        Ok(())
    }

    pub async fn read_from_file(file: &Path) -> Result<Self> {
        Ok(toml::from_slice::<Self>(&tokio::fs::read(file).await?)?)
    }

    pub fn iter(&self) -> impl Iterator<Item = Cookie> {
        self.cookies
            .iter()
            .map(|c| Cookie::new(c.0.clone(), c.1.clone()))
    }
}
