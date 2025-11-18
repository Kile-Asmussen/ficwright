use thirtyfour::{
    By, WebElement,
    components::{Component, ElementResolver},
};

use crate::{
    config::Fanfiction,
    forms::{associations_form::AssociationsForm, tags_form::TagsForm},
};
use crate::{forms::preface_form::PrefaceForm, *};

#[derive(Debug, Clone, Component)]
pub struct WorkForm {
    base: WebElement,

    #[by(css = "fieldset.work.meta")]
    pub tags: ElementResolver<TagsForm>,

    #[by(css = "fieldset.preface")]
    pub preface: ElementResolver<PrefaceForm>,

    #[by(css = "fieldset#associations")]
    pub associations: ElementResolver<AssociationsForm>,
}

impl WorkForm {
    pub async fn fill_out(&self, fic: &Fanfiction) -> Result<()> {
        self.tags.resolve().await?.set_all(&fic.tags).await?;
        self.preface.resolve().await?.set_all(&fic.fic).await?;
        Ok(())
    }
}
