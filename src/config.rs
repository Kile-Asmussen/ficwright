use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::{Path, PathBuf},
};

use clap::Parser;
use indexmap::IndexSet;
use rootcause::bail;
use serde::{Deserialize, Serialize};
use thirtyfour::Cookie;

use crate::{Result, forms::model::*, println_async, tree_set};

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct Fanfiction {
    pub fic: FicDetails,
    pub tags: FicTags,
    pub meta: FicMeta,
    pub remix: Option<FicRemix>,
    #[serde(default)]
    pub chapters: BTreeMap<String, FicDetails>,
}

impl Fanfiction {
    pub async fn load(path: &Path) -> Result<Self> {
        Ok(toml::from_str(&tokio::fs::read_to_string(path).await?)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FicTags {
    #[serde(default)]
    pub rating: AgeRating,
    pub warnings: BTreeSet<ArchiveWarning>,
    pub fandoms: IndexSet<String>,
    #[serde(default)]
    pub categories: BTreeSet<FicCategory>,
    #[serde(default)]
    pub relationships: IndexSet<String>,
    #[serde(default)]
    pub characters: IndexSet<String>,
    #[serde(default)]
    pub other: IndexSet<String>,
}

impl Default for FicTags {
    fn default() -> Self {
        Self {
            rating: Default::default(),
            warnings: tree_set![ArchiveWarning::CNTUAW],
            fandoms: Default::default(),
            categories: Default::default(),
            relationships: Default::default(),
            characters: Default::default(),
            other: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct FicDetails {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub author_pseud: Option<String>,
    #[serde(default)]
    pub co_authors: IndexSet<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub file: Option<PathBuf>,
    #[serde(default)]
    pub start_note: Option<String>,
    #[serde(default)]
    pub end_note: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct FicMeta {
    pub format: FileFormat,
    pub language: String,
    #[serde(default)]
    pub challenges: IndexSet<String>,
    #[serde(default)]
    pub gift_to: IndexSet<String>,
    #[serde(default)]
    pub work_skin: Option<String>,
    #[serde(default)]
    pub total_chapters: u64,
    #[serde(default)]
    pub in_series: Option<String>,
    #[serde(default)]
    pub publication_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct FicRemix {
    pub url: String,
    pub title: String,
    pub author: String,
    pub language: String,
    pub translated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieConfig {
    pub cookies: HashMap<String, String>,
}

impl CookieConfig {
    pub async fn save_to_file(&self, file: &Path) -> Result<()> {
        let file = if let Ok(f) = file.strip_prefix("~/") {
            let home = PathBuf::from(std::env::var("HOME")?);
            home.join(f)
        } else {
            file.to_path_buf()
        };
        tokio::fs::write(file, toml::to_string_pretty(&self)?).await?;
        Ok(())
    }

    pub async fn read_from_file(file: &Path) -> Result<Self> {
        println_async!("Reading cookie file: {}", file.to_string_lossy());
        let file = if let Ok(f) = file.strip_prefix("~/") {
            let home = PathBuf::from(std::env::var("HOME")?);
            home.join(f)
        } else {
            file.to_path_buf()
        };
        let contents = match tokio::fs::read_to_string(&file).await {
            Ok(ok) => ok,
            Err(e) => {
                println_async!("Fauled to read cookie file: {}", e.kind());
                bail!(e)
            }
        };
        Ok(match toml::from_str::<Self>(&contents) {
            Ok(cookies) => cookies,
            Err(e) => {
                println_async!("Failed to read cookie file: {}", e.message());
                bail!(e)
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = Cookie> {
        self.cookies
            .iter()
            .map(|c| Cookie::new(c.0.clone(), c.1.clone()))
    }

    pub fn new(cookies: Vec<Cookie>) -> Self {
        let cookies = cookies.into_iter().map(|c| (c.name, c.value)).collect();

        CookieConfig { cookies }
    }
}
