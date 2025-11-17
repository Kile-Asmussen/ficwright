use std::path::PathBuf;

use clap::Parser;

use std::default::Default;

use crate::{command::*, config::*, *};

#[derive(Debug, Clone, Parser)]
pub struct TemplateCommand {
    file: PathBuf,
}

impl Runnable for TemplateCommand {
    async fn run(self, _opts: FicwrightOpts) -> Result<()> {
        tokio::fs::write(
            self.file,
            toml::to_string_pretty(&Fanfiction {
                fic: FicDetails {
                    url: "".into(),
                    title: "Untitled".into(),
                    file: Some("untitled.md".into()),
                    start_note: Some("".into()),
                    end_note: Some("".into()),
                    summary: Some("Presented without summary".into()),
                },
                tags: Default::default(),
                meta: Default::default(),
                chapters: tree_map! {
                    "01".to_string() => Default::default(),
                    "02".to_string() => Default::default(),
                },
            })?,
        )
        .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Parser)]
pub struct DebugTemplateCommand {
    file: PathBuf,
}

impl Runnable for DebugTemplateCommand {
    async fn run(self, _opts: FicwrightOpts) -> Result<()> {
        println_async!(
            "{:#?}",
            toml::from_str::<Fanfiction>(&tokio::fs::read_to_string(&self.file).await?)?
        )
        .await;
        Ok(())
    }
}
