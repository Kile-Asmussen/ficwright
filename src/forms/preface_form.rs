use indexmap::IndexSet;
use thirtyfour::{
    By, WebElement,
    components::{Component, ElementResolver},
};

use crate::{
    config::FicDetails,
    forms::model::FileFormat,
    forms::{Autocomplete, Checkbox, DropdownSelector, PseudsSelector, TextField},
    *,
};

#[derive(Debug, Clone, Component)]
pub struct PrefaceForm {
    base: WebElement,

    #[by(css = "dd.title > input[type=\"text\"]")]
    title: ElementResolver<TextField>,

    #[by(css = "dd.summary > textarea")]
    summary: ElementResolver<TextField>,

    #[by(css = "dd.byline > select")]
    pseuds: ElementResolver<PseudsSelector>,

    #[by(css = "dd.byline.coauthors")]
    coauthors: ElementResolver<CoAuthors>,

    #[by(css = "dd.notes")]
    notes: ElementResolver<Notes>,
}

impl PrefaceForm {
    pub async fn set_all(&self, preface: &FicDetails) -> Result<()> {
        self.set_title(&preface.title).await?;

        self.pseuds.resolve().await?;

        self.set_cocreators(&preface.co_authors).await?;

        if let Some(summary) = &preface.summary {
            self.set_summary(&summary).await?;
        }

        let notes = self.notes.resolve().await?;
        notes
            .set_start(preface.start_note.as_ref().map(AsRef::as_ref))
            .await?;
        notes
            .set_end(preface.end_note.as_ref().map(AsRef::as_ref))
            .await?;

        Ok(())
    }

    pub async fn set_cocreators(&self, creators: &IndexSet<String>) -> Result<()> {
        println_async!("resloving");
        let x = self.coauthors.resolve().await?;
        println_async!("setting");
        x.set(creators).await
    }

    pub async fn set_title(&self, title: &str) -> Result<()> {
        let tit = self.title.resolve().await?;
        tit.delete_all().await?;
        tit.push_text(title, None).await?;
        Ok(())
    }

    pub async fn set_summary(&self, summary: &str) -> Result<()> {
        let sum = self.summary.resolve().await?;
        sum.delete_all().await?;
        sum.push_text(summary, None).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct CoAuthors {
    base: WebElement,

    #[by(css = "input[type=\"checkbox\"]")]
    has_coauthors: ElementResolver<Checkbox>,

    #[by(css = "fieldset > ul.autocomplete")]
    coauthors: ElementResolver<Autocomplete>,
}

impl CoAuthors {
    pub async fn set(&self, authors: &IndexSet<String>) -> Result<()> {
        let has = self.has_coauthors.resolve().await?;
        let open = has.state().await?;
        if authors.len() == 0 && open {
            self.coauthors.resolve().await?.delete_all().await?;
            has.set(false).await?;
        } else if authors.len() != 0 && !open {
            has.set(true).await?;
            let x = self.coauthors.resolve().await?;
            x.set(authors).await?;
        } else if authors.len() != 0 && open {
            self.coauthors.resolve().await?.set(authors).await?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct Notes {
    base: WebElement,

    #[by(css = "li.start > input[type=\"checkbox\"]")]
    has_startnote: ElementResolver<Checkbox>,
    #[by(css = "li.start > fieldset.start > textarea")]
    startnote: ElementResolver<TextField>,
    #[by(css = "li.end > input[type=\"checkbox\"]")]
    has_endnote: ElementResolver<Checkbox>,
    #[by(css = "li.end > fieldset.end > textarea")]
    endnote: ElementResolver<TextField>,
}

impl Notes {
    pub async fn set_start(&self, note: Option<&str>) -> Result<()> {
        let check = self.has_startnote.resolve().await?;
        self.set(&self.startnote, check, note).await
    }

    pub async fn set_end(&self, note: Option<&str>) -> Result<()> {
        let check = self.has_endnote.resolve().await?;
        self.set(&self.endnote, check, note).await
    }

    async fn set(
        &self,
        text: &ElementResolver<TextField>,
        check: Checkbox,
        note: Option<&str>,
    ) -> Result<()> {
        let state = check.state().await?;
        if note.is_none() && state {
            let text = text.resolve().await?;
            text.delete_all().await?;
            check.set(false).await?;
        } else if let Some(note) = note
            && state
        {
            let text = text.resolve().await?;
            text.delete_all().await?;
            text.push_text(note, None).await?;
        } else if let Some(note) = note
            && !state
        {
            check.set(true).await?;
            let text = text.resolve().await?;
            text.push_text(note, None).await?;
        }

        Ok(())
    }
}
