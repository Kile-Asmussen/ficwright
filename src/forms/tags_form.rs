use std::collections::BTreeSet;

use indexmap::IndexSet;
use strum::VariantArray;
use thirtyfour::{
    By, WebElement,
    components::{Component, ElementResolver},
};

use crate::{
    config::FicTags,
    forms::model::{AgeRating, ArchiveWarning, FicCategory},
    forms::{Autocomplete, CheckboxesByValue, DropdownSelector},
    *,
};

#[derive(Debug, Clone, Component)]
pub struct TagsForm {
    base: WebElement,
    #[by(css = "dd.rating > select")]
    work_rating: ElementResolver<DropdownSelector>,

    #[by(css = "dd.category > fieldset")]
    categories: ElementResolver<CheckboxesByValue>,

    #[by(css = "dd.warning > fieldset.warnings")]
    warnings: ElementResolver<CheckboxesByValue>,

    #[by(css = "dd.fandom > ul.autocomplete")]
    fandoms: ElementResolver<Autocomplete>,

    #[by(css = "dd.relationship > ul.autocomplete")]
    relationships: ElementResolver<Autocomplete>,

    #[by(css = "dd.character > ul.autocomplete")]
    characters: ElementResolver<Autocomplete>,

    #[by(css = "dd.freeform > ul.autocomplete")]
    other_tags: ElementResolver<Autocomplete>,
}

impl TagsForm {
    pub async fn demonstrate(&self) -> Result<()> {
        self.work_rating
            .resolve()
            .await?
            .demonstrate_by_value(AgeRating::VARIANTS)
            .await?;

        self.warnings.resolve().await?.demonstrate().await?;

        self.fandoms
            .resolve()
            .await?
            .demonstrate(&[
                "Bleach (Anime & Manga)",
                "Naruto (Anime & Manga)",
                "Pride and Prejudice - Jane Austen",
            ])
            .await?;

        self.categories.resolve().await?.demonstrate().await?;

        self.relationships
            .resolve()
            .await?
            .demonstrate(&[
                "Kurosaki Ichigo/Kuchiki Rukia",
                "Hyuuga Hinata/Uzumaki Naruto",
                "Mr. Darcy",
            ])
            .await?;

        self.characters
            .resolve()
            .await?
            .demonstrate(&["Huey", "Dewey", "Louie"])
            .await?;

        self.other_tags
            .resolve()
            .await?
            .demonstrate(&[
                "Isn't this just so cool?",
                "Look ma, no hands!",
                "A",
                "B",
                "C",
            ])
            .await?;

        Ok(())
    }

    pub async fn set_all(&self, tags: &FicTags) -> Result<()> {
        self.set_work_rating(tags.rating).await?;
        self.set_warnings(&tags.warnings).await?;
        self.set_fandoms(&tags.fandoms).await?;
        self.set_categories(&tags.categories).await?;
        self.set_relationships(&tags.relationships).await?;
        self.set_characters(&tags.characters).await?;
        self.set_other_tags(&tags.other).await?;

        Ok(())
    }

    pub async fn get_tags(&self) -> Result<FicTags> {
        let mut res = FicTags::default();

        Ok(res)
    }

    pub async fn set_work_rating(&self, age_rating: AgeRating) -> Result<()> {
        self.work_rating
            .resolve()
            .await?
            .select_by_value(&age_rating)
            .await?;
        Ok(())
    }

    pub async fn set_categories(&self, cats: impl IntoIterator<Item = &FicCategory>) -> Result<()> {
        let boxes = self.categories.resolve().await?;
        boxes.set_all_to(false);
        for cat in cats {
            boxes.set_one_to(cat, true).await?;
        }

        Ok(())
    }

    pub async fn set_warnings(&self, warnings: &BTreeSet<ArchiveWarning>) -> Result<()> {
        let boxes = self.warnings.resolve().await?;
        boxes.set_all_to(false).await?;
        if warnings.len() == 0 {
            boxes.set_one_to(&ArchiveWarning::CNTUAW, true).await?;
        } else {
            for warn in warnings {
                boxes.set_one_to(warn, true).await?;
            }
        }
        Ok(())
    }

    pub async fn set_fandoms(&self, fandoms: &IndexSet<String>) -> Result<()> {
        self.fandoms.resolve().await?.set(fandoms).await
    }

    pub async fn set_characters(&self, characters: &IndexSet<String>) -> Result<()> {
        self.characters.resolve().await?.set(characters).await
    }

    pub async fn set_relationships(&self, relationships: &IndexSet<String>) -> Result<()> {
        self.relationships.resolve().await?.set(relationships).await
    }

    pub async fn set_other_tags(&self, other: &IndexSet<String>) -> Result<()> {
        self.other_tags.resolve().await?.set(other).await
    }
}
