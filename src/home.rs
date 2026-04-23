use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    PortfolioContext,
    elements::{project_card::ProjectCard, tag::Tag},
    prelude::*,
};

pub const TITLE: LocStr = LocStr::Loc {
    en_us: "Home",
    fr_fr: "Accueil",
};
const SUBTITLE: LocStr = LocStr::Loc {
    en_us: "Hello & welcome",
    fr_fr: "Bonjour & bienvenue",
};

#[component]
pub fn Home() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let portfolio = use_context::<PortfolioContext>();

    let msg = use_memo(move || match &*portfolio.read() {
        Some(Ok(portfolio)) => portfolio
            .global_config
            .home_message
            .resolve(lang())
            .to_string(),
        _ => String::new(),
    });

    let mut filters = use_signal(HashSet::<String>::new);

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
                "{SUBTITLE.resolve(lang())}"
            }

            p {
                id: "message",
                "{msg}"
            }

            match &*portfolio.read() {
                Some(Ok(portfolio)) => rsx! {
                    div {
                        id: "filters",

                        for (name, tag) in portfolio
                            .tags
                            .iter()
                            .sorted_by(|(_, a), (_, b)| a.order.cmp(&b.order))
                            .map(|(name, tag)| (name.clone(), tag))
                        {
                            Tag {
                                tag_id: name.clone(),
                                tag: tag.clone(),
                                onclick: move |_| {
                                    let mut f = filters.write();
                                    f.toggle(name.clone());
                                },
                                active_list: ReadSignal::new(filters),
                            }
                        }
                    }

                    div {
                        id: "project-list",

                        for (id, project) in portfolio.projects.iter().sorted_by(|(_, a), (_, b)| a.order.cmp(&b.order)) {
                            ProjectCard {
                                project_id: id.clone(),
                                project: project.clone(),
                            }
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
