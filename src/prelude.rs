#![allow(unused_imports)]

pub use dioxus::prelude::*;

pub(crate) use crate::{
    data::{PortfolioDataView, ProjectView, TagView},
    utils::{
        color::PortfolioColor,
        error::{Error, Result, ResultExt},
        loc_string::{Lang, LocString},
    },
    PortfolioContext, Route,
};
