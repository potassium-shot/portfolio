use std::{path::PathBuf, sync::LazyLock};

use crate::prelude::*;

static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::CONFIG_PATH.join("projects"));

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Project {
    pub name: LocString,
    pub technologies: Vec<String>,
    pub tags: Vec<String>,
    pub short_description: LocString,
    pub description: LocString,
}

impl Project {
    pub fn load_all() -> Result<Vec<Self>> {
        super::load_all_in_dir(&PATH)
    }
}
