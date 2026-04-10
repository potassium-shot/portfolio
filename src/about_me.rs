use crate::prelude::*;

pub const TITLE: LocStr = LocStr::Loc {
    en_us: "About Me",
    fr_fr: "À propos",
};

#[component]
pub fn AboutMe() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let portfolio = use_context::<PortfolioContext>();

    rsx! {
        div {
            id: "about-me",
        }

        h1 {
            "{TITLE.resolve(lang())}"
        }

        match &*portfolio.read() {
            Some(Ok(portfolio)) => rsx! {
                p {
                    "{portfolio.global_config.about_me.resolve(lang())}"
                }
            },
            Some(Err(e)) => rsx! {
                p {
                    id: "error",
                    "Couldn't retrieve portfolio data: {e}"
                }
            },
            None => rsx! {
                p {
                    "..."
                }
            },
        }
    }
}
