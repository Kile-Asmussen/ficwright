use thirtyfour::components::{Component, ElementResolver};

use thirtyfour::{By, WebElement};

use crate::Result;
use crate::config::{FicDetails, FicMeta};
use crate::forms::{Autocomplete, AutocompleteEntry, Checkbox, DropdownSelector};

#[derive(Debug, Clone, Component)]
pub struct AssociationsForm {
    base: WebElement,

    #[by(css = "dd.collection > ul.autocomplete")]
    collections: ElementResolver<Autocomplete>,

    #[by(css = "dd.recipient > ul.recipient")]
    gift_to: ElementResolver<Autocomplete>,

    #[by(css = "dt.parent > input[type=\"checkbox\"]")]
    is_remixed: ElementResolver<Checkbox>,
    #[by(css = "dd.parent")]
    remix: ElementResolver<RemixForm>,

    #[by(css = "dt.serial > input[type=\"checkbox\"]")]
    is_serial: ElementResolver<Checkbox>,
    #[by(css = "dd.serial")]
    serial: ElementResolver<SerialForm>,

    #[by(css = "dt.chaptered.wip > input[type=\"checkbox\"]")]
    is_chaptered: ElementResolver<Checkbox>,
    #[by(css = "dd..chaptered.wip")]
    chaptered: ElementResolver<ChapteredForm>,

    #[by(css = "dt.backdate > input[type=\"checkbox\"]")]
    is_backdated: ElementResolver<Checkbox>,
    #[by(css = "dd.backdate")]
    backdating: ElementResolver<BackdatingForm>,

    #[by(css = "dd.language > select")]
    language: ElementResolver<DropdownSelector>,

    #[by(css = "dd.skin > select")]
    skin: ElementResolver<DropdownSelector>,
}

impl AssociationsForm {
    pub async fn set_all(&self, fic: &FicDetails, meta: &FicMeta) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct ChapteredForm {
    base: WebElement,
}

#[derive(Debug, Clone, Component)]
pub struct SerialForm {
    base: WebElement,
}

#[derive(Debug, Clone, Component)]
pub struct BackdatingForm {
    base: WebElement,
}

#[derive(Debug, Clone, Component)]
pub struct RemixForm {
    base: WebElement,
}
