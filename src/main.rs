use crate::{data::PortfolioData, prelude::*};

mod api;
mod assets;
mod components;
mod data;
mod elements;
mod home;
mod navbar;
mod prelude;
mod project_page;
mod utils;

use home::Home;
use project_page::ProjectPage;

#[derive(Routable, kinded::Kinded, Clone, PartialEq, Eq, Debug)]
#[rustfmt::skip]
enum Route {
    #[layout(navbar::PortfolioNavbar)]
        #[route("/")]
        Home {},

        #[route("/project/:project")]
        ProjectPage { project: String },
}

fn main() {
    #[cfg(feature = "server")]
    crate::data::image::lazy_load();

    dioxus::LaunchBuilder::new().launch(App);
}

type PortfolioContext = Resource<Result<PortfolioDataView, ServerFnError>>;

#[component]
fn App() -> Element {
    let portfolio: PortfolioContext = use_resource(move || async move {
        crate::api::fetch_portfolio_data()
            .await
            .map(PortfolioData::into_view)
    });
    use_context_provider(move || portfolio);
    use_context_provider(move || Lang::EnUs);

    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Chakra+Petch:ital,wght@0,300;0,400;0,500;0,600;0,700;1,300;1,400;1,500;1,600;1,700&display=swap",
        }
        document::Link { rel: "icon", href: assets::FAVICON }

        for css in assets::GLOBAL_CSS.iter().copied() {
            document::Link { rel: "stylesheet", href: css }
        }

        div {
            id: "background-banana",
            style: "mask-image: url({assets::BANANA.resolve().to_string_lossy().into_owned()})",
        }

        Router::<Route> {}
    }
}
