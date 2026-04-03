use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::prelude::*;

pub mod project;
pub mod tag;
pub mod technology;

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var("PORTFOLIO_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/usr/share/portfolio"))
});

fn load_all_in_dir<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Vec<T>> {
    Ok(std::fs::read_dir(path)
        .error_code(500)?
        .filter_map(|element| element.ok())
        .filter(|element| element.file_type().is_ok_and(|ty| ty.is_file()))
        .filter_map(|file| std::fs::read_to_string(file.path()).ok())
        .filter_map(|text| ron::from_str::<T>(text.as_str()).ok())
        .collect())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct PortfolioData {
    pub projects: Vec<project::Project>,
    pub technologies: Vec<technology::Technology>,
    pub tags: Vec<tag::Tag>,
}

impl PortfolioData {
    pub fn load() -> Result<Self> {
        Ok(Self {
            projects: project::Project::load_all()?,
            technologies: technology::Technology::load_all()?,
            tags: tag::Tag::load_all()?,
        })
    }
}
