#![allow(unused_imports)]

pub use dioxus::prelude::*;

pub(crate) use crate::{
    PortfolioContext, Route,
    data::{PortfolioDataView, ProjectView, TagView},
    dx_dbg,
    utils::{
        color::PortfolioColor,
        error::{Error, Result, ResultExt},
        ext::*,
        loc_string::{Lang, LocStr, LocString},
    },
};
