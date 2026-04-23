use crate::{
    elements::{carrousel::Carrousel, tag::Tag},
    prelude::*,
};

#[component]
pub fn ProjectCard(project_id: String, project: ProjectView) -> Element {
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
            id: "project-card",

            div {
                id: "text",

                Link {
                    id: "title",
                    to: Route::ProjectPage { project: project_id },
                    "{project.name.resolve(lang())}"
                }

                p {
                    for (tag_id, tag) in tags {
                        Tag {
                            tag_id: tag_id,
                            tag: tag,
                        }

                        span { " " }
                    }

                    span {
                        id: "short-description",
                        "{project.short_description.resolve(lang())}"
                    }
                }
            }

            Carrousel {
                images: project.images.clone(),
                size: "thumbnail",
            }
        }
    }
}
