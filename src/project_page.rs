use crate::{
    elements::{carrousel::Carrousel, tag::Tag},
    prelude::*,
};

pub const TITLE: LocStr = LocStr::Loc {
    en_us: "Projects",
    fr_fr: "Projets",
};

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
    let lang = use_context::<Signal<Lang>>();
    let portfolio = use_context::<PortfolioContext>();

    let tags = project.tags.iter().flat_map(|tag| {
        Some((
            tag.clone(),
            portfolio.unwrap().unwrap().find_tag(tag.as_str())?,
        ))
    });

    rsx! {
        div {
            id: "project-page",

            div {
                id: "title-bar",

                h1 {
                    id: "title",
                    "{project.name.resolve(lang())}"
                }

                span {
                    id: "tags",

                    for (tag_id, tag) in tags {
                        Tag {
                            tag_id: tag_id,
                            tag: tag,
                        }
                    }
                }
            }

            Carrousel {
                images: project.images.clone(),
                size: "original",
            }

            div {
                id: "description",
                dangerous_inner_html: "{project.description.resolve(lang())}",
            }
        }
    }
}
