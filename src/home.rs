use itertools::Itertools;

use crate::{PortfolioContext, elements::project_card::ProjectCard, prelude::*};

#[component]
pub fn Home() -> Element {
    let lang = use_context::<Lang>();
    let portfolio = use_context::<PortfolioContext>();

    let msg = use_memo(move || match &*portfolio.read() {
        Some(Ok(portfolio)) => portfolio
            .global_config
            .home_message
            .resolve(lang)
            .to_string(),
        _ => String::new(),
    });

    rsx! {
        document::Link { rel: "stylesheet", href: crate::assets::HOME_CSS }
        div {
            id: "top-section",

            h1 {
                id: "title",
                "Potassium Shot"
            }
            h3 {
                id: "subtitle",
                "Hello & welcome"
            }

            p {
                id: "message",
                "{msg}"
            }

            div {
                id: "project-list",

                match &*portfolio.read() {
                    Some(Ok(portfolio)) => rsx! {
                        for (id, project) in portfolio.projects.iter().sorted_by(|(_, a), (_, b)| a.order.cmp(&b.order)) {
                            ProjectCard {
                                project_id: id.clone(),
                                project: project.clone(),
                            }
                        }
                    },
                    Some(Err(error)) => rsx! { p {
                        id: "error",
                        "Error, couldn't retrieve portfolio data.\n{error}"
                    } },
                    None => rsx! {
                        for _ in 0..4 {
                            div {
                                id: "project-card-placeholder",
                            }
                        }
                    },
                }
            }
        }
    }
}
