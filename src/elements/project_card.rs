use crate::{elements::tag::Tag, prelude::*};

#[component]
pub fn ProjectCard(project: ProjectView) -> Element {
    let lang = use_context::<Lang>();
    let portfolio = use_context::<PortfolioContext>();

    let tags = project
        .tags
        .iter()
        .flat_map(|tag| portfolio.unwrap().unwrap().find_tag(tag.as_str()));

    rsx! {
        div {
            id: "project-card",

            h3 { "{project.name.resolve(lang)}" }

            p {
                for tag in tags {
                    Tag {
                        tag: tag,
                    }
                }

                span {
                    id: "short-description",
                    "{project.short_description.resolve(lang)}"
                }
            }
        }
    }
}
