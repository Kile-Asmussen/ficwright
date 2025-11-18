use std::time::Duration;

use indexmap::{IndexMap, IndexSet};
use rootcause::prelude::*;
use strum::VariantArray;
use thirtyfour::{
    By, Key, WebElement,
    components::{Component, ElementResolver},
};

pub mod associations_form;
pub mod model;
pub mod preface_form;
pub mod tags_form;
pub mod work_form;

use crate::*;
use model::UseByValue;

#[derive(Debug, Clone, Component)]
pub struct DropdownSelector {
    base: WebElement,
    #[by(tag = "option")]
    options: ElementResolver<Vec<WebElement>>,
}

impl DropdownSelector {
    pub async fn current_value(&self) -> Result<Option<String>> {
        let selected = Some("true".to_string());
        for opt in self.options.resolve().await? {
            if opt.prop("selected").await? == selected {
                return Ok(Some(opt.value().await?.unwrap_or_default()));
            }
        }
        return Ok(None);
    }

    pub async fn list_all_by_value(&self) -> Result<Vec<(String, bool)>> {
        let mut res = vec![];
        let selected = Some("true".to_string());
        for opt in self.options.resolve().await? {
            res.push((
                opt.value().await?.unwrap_or_default(),
                opt.prop("selected").await? == selected,
            ));
        }
        Ok(res)
    }

    pub async fn list_all_by_innerhtml(&self) -> Result<Vec<(String, bool)>> {
        let mut res = vec![];
        let selected = Some("true".to_string());
        for opt in self.options.resolve().await? {
            res.push((
                opt.inner_html().await?,
                opt.prop("selected").await? == selected,
            ));
        }
        Ok(res)
    }

    pub async fn select_by_value<T: UseByValue>(&self, val: &T) -> Result<bool> {
        self.base.click().await?;
        let options = self.options.resolve().await?;
        let ref_value = val.as_value();
        for option in options {
            let value = option.value().await?;
            if (&value).as_ref() == Some(&ref_value) {
                option.click().await?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn select_by_innerhtml(&self, val: &str) -> Result<bool> {
        self.base.click().await?;
        let options = self.options.resolve().await?;
        for option in options {
            let value = option.inner_html().await?;
            if value == val {
                option.click().await?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn demonstrate_by_value<T: UseByValue>(&self, vals: &[T]) -> Result<()> {
        self.base.scroll_into_view().await?;

        let current = self.current_value().await?.unwrap_or_default();

        println_async!("DropdownSelect: {:#?}", self.list_all_by_value().await?);

        for val in vals {
            self.select_by_value(val).await?;
            tokio::time::sleep(Duration::from_millis(300));
        }

        self.select_by_value(&current).await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct CheckboxesByValue {
    base: WebElement,
    #[by(css = "input[type=\"checkbox\"]")]
    boxes: ElementResolver<Vec<Checkbox>>,
}

#[derive(Debug, Clone, Component)]
pub struct Checkbox {
    base: WebElement,
}

impl Checkbox {
    pub async fn value(&self) -> Result<String> {
        Ok(self.base.value().await?.unwrap_or_default())
    }

    pub async fn state(&self) -> Result<bool> {
        Ok(self.base.prop("checked").await? == Some("true".to_string()))
    }

    pub async fn set(&self, state: bool) -> Result<()> {
        if self.base.is_clickable().await? && self.state().await? != state {
            self.base.click().await?;
        }
        Ok(())
    }

    pub async fn blink(&self) -> Result<()> {
        self.base.scroll_into_view().await?;
        let curr = self.state().await?;
        let mut x = !curr;
        for _ in 0..3 {
            self.set(x).await?;
            x = !x;
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        self.set(curr).await?;

        Ok(())
    }
}

impl CheckboxesByValue {
    pub async fn all_by_values(&self) -> Result<IndexMap<String, bool>> {
        let mut res = ix_map! {};

        for elem in self.boxes.resolve().await? {
            res.insert(elem.value().await?, elem.state().await?);
        }

        Ok(res)
    }

    pub async fn set_one_to<T: UseByValue>(&self, id: &T, state: bool) -> Result<bool> {
        let id = id.as_value();
        for elem in self.boxes.resolve().await? {
            if elem.value().await? == id {
                elem.set(state).await?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn set_all_to(&self, state: bool) -> Result<()> {
        for elem in self.boxes.resolve().await? {
            elem.set(state).await?;
        }
        Ok(())
    }

    pub async fn demonstrate(&self) -> Result<()> {
        self.base.scroll_into_view().await?;
        let curr = self.all_by_values().await?;

        println_async!("CheckboxesByValue: {:#?}", curr);

        self.set_all_to(false).await;
        tokio::time::sleep(Duration::from_millis(100));
        self.set_all_to(true).await;
        tokio::time::sleep(Duration::from_millis(100));
        self.set_all_to(false).await;

        for (val, state) in curr {
            self.set_one_to(&val, state);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct TextField {
    base: WebElement,
}

impl TextField {
    pub async fn push_text(&self, text: &str, end: Option<Key>) -> Result<()> {
        self.base.focus().await?;
        self.base.send_keys(text).await?;
        if let Some(k) = end {
            self.base.send_keys(k).await?;
        }
        Ok(())
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.base.clear().await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct Autocomplete {
    base: WebElement,
    #[by(css = "li.input > input.text[type=\"text\"]")]
    text_field: ElementResolver<TextField>,
    // bugged: this just doesn't work for some reason
    // #[by(allow_empty, css = "li.added.tag")]
    // entries: ElementResolver<Vec<AutocompleteEntry>>,
}

impl Autocomplete {
    pub async fn demonstrate(&self, entries: &[&str]) -> Result<()> {
        let current = self.list_entries().await?;

        self.delete_all().await?;

        for entry in entries {
            tokio::time::sleep(Duration::from_millis(250)).await;
            self.add_entry(entry).await?;
        }

        tokio::time::sleep(Duration::from_millis(250)).await;

        self.delete_all().await?;

        for entry in current {
            self.add_entry(&entry).await?;
        }

        Ok(())
    }

    pub async fn entries(&self) -> Result<Vec<AutocompleteEntry>> {
        // Ok(self.entries.resolve().await?)
        let all = self.base.find_all(By::Css("li.added.tag")).await?;
        let mut res = vec![];
        for e in all {
            res.push(AutocompleteEntry::from(e))
        }
        Ok(res)
    }

    pub async fn add_entry(&self, entry: &str) -> Result<()> {
        self.text_field
            .resolve()
            .await?
            .push_text(entry, Some(Key::Tab))
            .await?;
        Ok(())
    }

    pub async fn list_entries(&self) -> Result<Vec<String>> {
        let mut res = vec![];

        let entries = self.entries().await?;

        for entry in entries {
            println_async!("getting entry name");
            let name = entry.entry_name().await?;
            res.push(name);
        }

        Ok(res)
    }

    pub async fn delete_all(&self) -> Result<()> {
        for entry in self.entries().await? {
            entry.delete().await?;
        }
        Ok(())
    }

    pub async fn delete_entry(&self, name: &str) -> Result<()> {
        for entry in self.entries().await? {
            if entry.entry_name().await? == name {
                entry.delete().await?;
            }
        }
        Ok(())
    }

    pub async fn set(&self, strings: &IndexSet<String>) -> Result<()> {
        let mut all = IndexSet::<String>::from_iter(self.list_entries().await?);

        for entry in all.difference(strings) {
            self.delete_entry(entry).await?;
        }

        for entry in strings.difference(&all) {
            self.add_entry(entry).await?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct AutocompleteEntry {
    base: WebElement,
    #[by(css = "span.delete > a")]
    delete: ElementResolver<WebElement>,
}

impl AutocompleteEntry {
    pub async fn entry_name(&self) -> Result<String> {
        let re = regex::Regex::new(r#"(.*)\s+<span class="delete">"#).unwrap();
        if let Some(cap) = re.captures(&(self.base.inner_html().await?)) {
            return Ok(cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default());
        }
        return Ok("".to_string());
    }

    pub async fn delete(&self) -> Result<()> {
        self.delete.resolve().await?.click().await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct PseudsSelector {
    base: WebElement,
}

impl PseudsSelector {}
