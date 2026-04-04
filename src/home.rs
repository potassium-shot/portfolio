use crate::{elements::project_card::ProjectCard, prelude::*, PortfolioContext};

#[component]
pub fn Home() -> Element {
    let portfolio = use_context::<PortfolioContext>();

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
                "Portfolio"
            }

            match &*portfolio.read() {
                Some(Ok(portfolio)) => rsx! {
                    for project in portfolio.projects.values().cloned() {
                        ProjectCard {
                            project: project,
                        }
                    }
                },
                Some(Err(error)) => rsx! { p {
                    id: "error",
                    "Error, couldn't retrieve portfolio data.\n{error}"
                } },
                None => rsx! { p { "Loading heehee" } },
            }
        }
    }
}
