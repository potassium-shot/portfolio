use crate::{elements::carrousel::Carrousel, prelude::*};

#[component]
pub fn ProjectPage(project: String) -> Element {
    let portfolio = use_context::<PortfolioContext>();

    rsx! {
        document::Link { rel: "stylesheet", href: crate::assets::PROJECT_PAGE_CSS }

        match &*portfolio.read() {
            Some(Ok(portfolio)) => match portfolio.find_project(project.as_str()) {
                Some(project) => rsx! { ProjectPageContent { project: project } },
                None => rsx! {
                    p {
                        id: "error",
                        "Unknown project '{project}'."
                    }
                },
            },
            Some(Err(e)) => rsx! {
                p {
                    id: "error",
                    "{e}"
                }
            },
            None => rsx! { p { "PLEASE WAIT THANK YOUUU" } },
        }
    }
}

#[component]
fn ProjectPageContent(project: ProjectView) -> Element {
    let lang = use_context::<Lang>();

    rsx! {
        div {
            id: "project-page",

            h1 {
                id: "title",
                "{project.name.resolve(lang)}"
            }

            Carrousel {
                images: project.images.clone(),
                size: "original",
            }

            p {
                id: "description",
                "{project.description.resolve(lang)}"
            }
        }
    }
}
