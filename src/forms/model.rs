use std::process::Stdio;

use crate::*;
use clap::Parser;
use rootcause::report;
use serde::{Deserialize, Serialize};
use strum::VariantArray;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Parser, VariantArray)]
pub enum FileFormat {
    Typst,
    #[default]
    Markdown,
    HTML,
}

impl FileFormat {
    pub async fn to_html(self, string: &str) -> Result<String> {
        match self {
            Self::Typst => todo!(),
            Self::Markdown => Self::parse_markdown(string).await,
            Self::HTML => Ok(string.to_string()),
        }
    }

    pub async fn parse_markdown(string: &str) -> Result<String> {
        let mut pandoc = Command::new("pandoc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let mut cin = pandoc
            .stdin
            .take()
            .ok_or_else(|| report!("No stdin on pandoc"))?;

        let cout = pandoc
            .stdout
            .take()
            .ok_or_else(|| report!("No stdout on pandoc"))?;

        cin.write_all(&string.as_bytes()).await?;
        cin.shutdown().await?;

        let mut res = String::new();
        let mut cout = BufReader::new(cout);
        cout.read_to_string(&mut res).await?;

        pandoc.kill().await?;

        Ok(res)
    }
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, VariantArray,
)]
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

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    VariantArray,
)]
pub enum ArchiveWarning {
    #[serde(rename = "Chose Not To Use Archive Warnings")]
    #[default]
    CNTUAW,
    #[serde(rename = "Graphic Depictions Of Violence")]
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

#[derive(
    Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash, VariantArray,
)]
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

pub fn enum_as_string<T: Serialize>(e: &T) -> String {
    let mut buf = String::new();
    e.serialize(toml::ser::ValueSerializer::new(&mut buf))
        .unwrap();
    let val = buf.parse::<toml::Value>().unwrap();
    match val {
        toml::Value::String(s) => return s,
        val => panic!("Not a string: {}", val),
    }
}

pub trait UseByValue {
    fn as_value(&self) -> String;
}

impl UseByValue for AgeRating {
    fn as_value(&self) -> String {
        enum_as_string(self)
    }
}

impl UseByValue for ArchiveWarning {
    fn as_value(&self) -> String {
        enum_as_string(self)
    }
}

impl UseByValue for FicCategory {
    fn as_value(&self) -> String {
        enum_as_string(self)
    }
}

impl UseByValue for String {
    fn as_value(&self) -> String {
        self.clone()
    }
}
