#[cfg(feature = "server")]
use std::collections::HashMap;
use std::{path::PathBuf, sync::LazyLock};

use crate::prelude::*;

static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::CONFIG_PATH.join("projects"));

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Project {
    pub name: LocString,
    pub tags: Vec<String>,
    pub short_description: LocString,
    pub images: Vec<String>,
    pub description: LocString,
}

impl Project {
    #[cfg(feature = "server")]
    pub fn load_all() -> Result<HashMap<String, Self>> {
        super::load_all_in_dir(&PATH)
    }
}
