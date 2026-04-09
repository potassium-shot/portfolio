use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};

use itertools::Itertools;

use crate::prelude::*;

#[cfg(feature = "server")]
pub mod image;
pub mod project;
pub mod tag;

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var("PORTFOLIO_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/usr/share/portfolio"))
});

#[cfg(feature = "server")]
fn load_all_in_dir<T: serde::de::DeserializeOwned>(path: &Path) -> Result<HashMap<String, T>> {
    Ok(std::fs::read_dir(path)
        .error_code(500)?
        .filter_map(|element| element.ok())
        .filter(|element| element.file_type().is_ok_and(|ty| ty.is_file()))
        .filter_map(|file| {
            std::fs::read_to_string(file.path()).ok().map(|text| {
                (
                    file.path()
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    text,
                )
            })
        })
        .filter_map(|(id, text)| ron::from_str::<T>(text.as_str()).ok().map(|t| (id, t)))
        .collect())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct PortfolioData {
    pub projects: HashMap<String, project::Project>,
    pub tags: HashMap<String, tag::Tag>,
}

impl PortfolioData {
    #[cfg(feature = "server")]
    pub fn load() -> Result<Self> {
        Ok(Self {
            projects: project::Project::load_all()?,
            tags: tag::Tag::load_all()?,
        })
    }

    pub fn into_view(self) -> PortfolioDataView {
        PortfolioDataView {
            projects: Arc::new(
                self.projects
                    .into_iter()
                    .map(|(id, project)| (id, Arc::new(project)))
                    .collect(),
            ),
            tags: Arc::new(
                self.tags
                    .into_iter()
                    .map(|(id, tag)| (id, Arc::new(tag)))
                    .sorted_by(|(_, a), (_, b)| a.order.cmp(&b.order))
                    .collect(),
            ),
        }
    }
}

pub type ProjectView = Arc<project::Project>;
pub type TagView = Arc<tag::Tag>;

#[derive(Clone, Debug)]
pub struct PortfolioDataView {
    pub projects: Arc<HashMap<String, ProjectView>>,
    pub tags: Arc<HashMap<String, TagView>>,
}

impl PortfolioDataView {
    pub fn find_tag(&self, tag: &str) -> Option<TagView> {
        self.tags.get(tag).cloned()
    }
}
