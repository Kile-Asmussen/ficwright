use std::{path::PathBuf, time::Duration};

use clap::Parser;
use rootcause::bail;
use thirtyfour::{
    By, Key, WebElement,
    components::{Component, ElementResolver},
};

use crate::{
    command::{Ao3Opts, WebRunnable},
    config::{AgeRating, ArchiveWarning, Fanfiction, FicCategory, FicTags, enum_as_string},
    driver::DriverExts,
    utils::prompt,
    *,
};

#[derive(Debug, Clone, Parser)]
pub struct Ao3PostNew {
    fic: PathBuf,

    #[clap(long)]
    draft: bool,

    #[clap(skip)]
    loaded: Fanfiction,
}

impl WebRunnable for Ao3PostNew {
    async fn pre(&mut self, _opts: &Ao3Opts) -> Result<()> {
        self.loaded = toml::from_str(&tokio::fs::read_to_string(&self.fic).await?)?;
        println_async!("Loaded: {:#?}", &self.loaded).await;
        Ok(())
    }

    async fn run(self, driver: &mut thirtyfour::WebDriver, opt: Ao3Opts) -> Result<()> {
        driver.add_cookies(&opt.get_cookies().await?).await?;

        driver.ao3("/works/new").await?;

        let work_form = driver.find(By::Id("work-form")).await?;

        let work_form = PostNewForm::from(work_form);

        work_form.set_tags(&self.loaded.tags).await?;

        prompt("Enter to continue...").await;

        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct PostNewForm {
    base: WebElement,
    #[by(id = "work_rating_string")]
    work_rating: ElementResolver<SelectByValue>,

    #[by(id = "work_fandom_autocomplete")]
    fandoms: ElementResolver<Autocomplete>,
    #[by(id = "work_relationship_autocomplete")]
    relationships: ElementResolver<Autocomplete>,
    #[by(id = "work_character_autocomplete")]
    characters: ElementResolver<Autocomplete>,
    #[by(id = "work_freeform_autocomplete")]
    other_tags: ElementResolver<Autocomplete>,

    #[by(css = "dd.category > fieldset")]
    category: ElementResolver<Checkboxes>,

    #[by(css = "dd.warning > fieldset.warnings")]
    warnings: ElementResolver<Checkboxes>,
}

impl PostNewForm {
    pub async fn set_tags(&self, tags: &FicTags) -> Result<()> {
        print_async!("Setting work rating").await;
        self.set_work_rating(tags.rating).await?;

        println_async!("Adding {} warnings", tags.warnings.len()).await;
        for warn in &tags.warnings {
            self.set_warning(*warn).await?;
        }

        if tags.warnings.is_empty() {
            self.set_warning(ArchiveWarning::CNTUAW).await?;
        }

        println_async!("Adding {} fandoms", tags.fandoms.len()).await;
        for fandom in &tags.fandoms {
            self.add_fandom(fandom).await?;
        }

        println_async!("Adding {} categories", tags.categories.len()).await;
        for cat in &tags.categories {
            self.set_category(*cat).await?;
        }

        println_async!("Adding {} relationships", tags.relationships.len()).await;
        for ship in &tags.relationships {
            self.add_relationship(ship).await?;
        }

        println_async!("Adding {} characters", tags.characters.len()).await;
        for character in &tags.characters {
            self.add_character(character).await?;
        }

        println_async!("Adding {} misc tags", tags.other.len()).await;
        for tag in &tags.other {
            self.add_other_tag(tag).await?;
        }

        Ok(())
    }

    pub async fn set_work_rating(&self, age_rating: AgeRating) -> Result<()> {
        let rating = self.work_rating.resolve().await?;
        rating.select(enum_as_string(&age_rating)?).await?;
        Ok(())
    }

    pub async fn set_category(&self, cat: FicCategory) -> Result<()> {
        let boxes = self.category.resolve().await?;
        let cat = match cat {
            FicCategory::FF => "work_category_strings_ff",
            FicCategory::MM => "work_category_strings_mm",
            FicCategory::FM => "work_category_strings_fm",
            FicCategory::Gen => "work_category_strings_gen",
            FicCategory::Multi => "work_category_strings_multi",
            FicCategory::Other => "work_category_strings_other",
        };
        boxes.check_box(cat).await?;
        Ok(())
    }

    pub async fn set_warning(&self, warn: ArchiveWarning) -> Result<()> {
        let boxes = self.warnings.resolve().await?;
        let warn = match warn {
            ArchiveWarning::CNTUAW => {
                "work_archive_warning_strings_choose_not_to_use_archive_warnings"
            }
            ArchiveWarning::Violence => {
                "work_archive_warning_strings_graphic_depictions_of_violence"
            }
            ArchiveWarning::MCDeath => "work_archive_warning_strings_major_character_death",
            ArchiveWarning::NA => "work_archive_warning_strings_no_archive_warnings_apply",
            ArchiveWarning::NonCon => "work_archive_warning_strings_rapenon-con",
            ArchiveWarning::Underage => "work_archive_warning_strings_underage_sex",
        };
        boxes.check_box(warn).await?;
        Ok(())
    }

    pub async fn add_fandom(&self, fandom: &str) -> Result<()> {
        self.fandoms.resolve().await?.push_value(fandom).await?;
        Ok(())
    }

    pub async fn add_relationship(&self, relationship: &str) -> Result<()> {
        self.relationships
            .resolve()
            .await?
            .push_value(relationship)
            .await?;
        Ok(())
    }

    pub async fn add_character(&self, character: &str) -> Result<()> {
        self.characters
            .resolve()
            .await?
            .push_value(character)
            .await?;
        Ok(())
    }

    pub async fn add_other_tag(&self, tag: &str) -> Result<()> {
        self.other_tags.resolve().await?.push_value(tag).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct SelectByValue {
    base: WebElement,
    #[by(tag = "option")]
    options: ElementResolver<Vec<WebElement>>,
}

impl SelectByValue {
    pub async fn select(&self, s: String) -> Result<()> {
        self.base.click().await?;
        let ratings = self.options.resolve().await?;
        for option in ratings {
            if option.value().await?.as_ref() == Some(&s) {
                option.click().await?;
                return Ok(());
            }
        }
        bail!("Selectable with value {} not found", s)
    }
}

#[derive(Debug, Clone, Component)]
pub struct Autocomplete {
    base: WebElement,
}

impl Autocomplete {
    pub async fn push_value(&self, s: &str) -> Result<()> {
        self.base.focus().await?;
        self.base.send_keys(s).await?;
        self.base.send_keys(Key::Tab).await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct Checkboxes {
    base: WebElement,
}

impl Checkboxes {
    pub async fn check_box(&self, id: &str) -> Result<()> {
        let x = self.base.find(By::Id(id)).await?;
        // if x.is_clickable().await? && x.prop("checked").await?.unwrap_or_default() != "true" {
        x.click().await?;
        // }
        Ok(())
    }
}
