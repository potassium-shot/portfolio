use std::{path::PathBuf, sync::LazyLock};

use crate::prelude::*;

static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::CONFIG_PATH.join("tags"));

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Tag {
    pub name: LocString,
    pub color: PortfolioColor,
}

impl Tag {
    pub fn load_all() -> Result<Vec<Self>> {
        super::load_all_in_dir(&PATH)
    }
}
