use std::{path::PathBuf, sync::LazyLock};

use crate::prelude::*;

static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::CONFIG_PATH.join("technologies"));

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Technology {
    pub name: LocString,
    pub color: PortfolioColor,
}

impl Technology {
    pub fn load_all() -> Result<Vec<Technology>> {
        super::load_all_in_dir(&PATH)
    }
}
