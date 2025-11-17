use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::{Path, PathBuf},
};

use clap::Parser;
use indexmap::IndexSet;
use rootcause::bail;
use serde::{Deserialize, Serialize};
use thirtyfour::Cookie;

use crate::{Result, println_async, tree_set};

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct Fanfiction {
    pub fic: FicDetails,
    pub tags: FicTags,
    pub meta: FicMeta,
    #[serde(default)]
    pub chapters: BTreeMap<String, FicDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FicTags {
    #[serde(default)]
    pub rating: AgeRating,
    pub warnings: BTreeSet<ArchiveWarning>,
    pub fandoms: IndexSet<String>,
    #[serde(default)]
    pub categories: IndexSet<FicCategory>,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FicCategory {
    #[serde(rename = "F/F")]
    FF,
    #[serde(rename = "M/M")]
    MM,
    #[serde(rename = "F/M")]
    FM,
    Gen,
    Multi,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArchiveWarning {
    #[serde(rename = "Chose Not To Use Archive Warnings")]
    #[default]
    CNTUAW,
    #[serde(rename = "Graphic Depictions of Violence")]
    Violence,
    #[serde(rename = "Major Character Death")]
    MCDeath,
    #[serde(rename = "No Archive Warnings Apply")]
    NA,
    #[serde(rename = "Rape/Non-Con")]
    NonCon,
    #[serde(rename = "Underage Sex")]
    Underage,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgeRating {
    #[serde(rename = "Not Rated")]
    #[default]
    NotRated,
    #[serde(rename = "General Audiences")]
    GenAud,
    #[serde(rename = "Teen And Up Audiences")]
    TeenAud,
    #[serde(rename = "Mature")]
    MatureAud,
    #[serde(rename = "Explicit")]
    Explicit,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct FicDetails {
    #[serde(default)]
    pub url: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Parser)]
pub enum FileFormat {
    Typst,
    #[default]
    Markdown,
    HTML,
}

pub fn enum_as_string<T: Serialize>(e: &T) -> Result<String> {
    let mut buf = String::new();
    e.serialize(toml::ser::ValueSerializer::new(&mut buf))?;
    let val = buf.parse::<toml::Value>()?;
    match val {
        toml::Value::String(s) => return Ok(s),
        val => bail!("Not a string: {}", val),
    }
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
        println_async!("Reading cookie file: {}", file.to_string_lossy()).await;
        let file = if let Ok(f) = file.strip_prefix("~/") {
            let home = PathBuf::from(std::env::var("HOME")?);
            home.join(f)
        } else {
            file.to_path_buf()
        };
        let contents = match tokio::fs::read_to_string(&file).await {
            Ok(ok) => ok,
            Err(e) => {
                println_async!("Fauled to read cookie file: {}", e.kind()).await;
                bail!(e)
            }
        };
        Ok(match toml::from_str::<Self>(&contents) {
            Ok(cookies) => cookies,
            Err(e) => {
                println_async!("Failed to read cookie file: {}", e.message()).await;
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
