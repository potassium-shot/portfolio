#[cfg(feature = "server")]
use std::collections::HashMap;
use std::{path::PathBuf, sync::LazyLock};

use crate::prelude::*;

static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::CONFIG_PATH.join("tags"));

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Tag {
    pub name: LocString,
    pub color: PortfolioColor,
    pub order: i32,
}

impl Tag {
    #[cfg(feature = "server")]
    pub fn load_all() -> Result<HashMap<String, Self>> {
        super::load_all_in_dir(&PATH)
    }
}
