use crate::prelude::*;

pub const GLOBAL_CSS: &[Asset] = crate::css_assets![
    "chakra-petch",
    "main",
    "theme",
    "home",
    "project-card",
    "tags",
    "project-page",
    "carrousel",
];

pub const HOME_CSS: Asset = asset!("/assets/home.css");
pub const PROJECT_PAGE_CSS: Asset = asset!("/assets/project-page.css");

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const BANANA: Asset = asset!("/assets/banana-silhouette.svg");
pub const EN_US: Asset = asset!("/assets/lang/en-us.svg");
pub const FR_FR: Asset = asset!("/assets/lang/fr-fr.svg");
