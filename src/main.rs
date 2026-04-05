use crate::{data::PortfolioData, prelude::*};

mod api;
mod assets;
mod data;
mod elements;
mod home;
mod navbar;
mod prelude;
mod utils;

use home::Home;

#[derive(Routable, kinded::Kinded, Clone, PartialEq, Eq, Debug)]
#[rustfmt::skip]
enum Route {
    #[layout(navbar::Navbar)]
    #[route("/")]
    Home {},
}

fn main() {
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
        document::Link { rel: "stylesheet", href: assets::FONT_CSS }
        document::Link { rel: "stylesheet", href: assets::MAIN_CSS }
        document::Link { rel: "stylesheet", href: assets::THEME_CSS }
        document::Link { rel: "stylesheet", href: assets::PROJECT_CARD_CSS }
        document::Link { rel: "stylesheet", href: assets::TAGS_CSS }

        div {
            id: "background-banana",
            style: "mask-image: url({assets::BANANA.resolve().to_string_lossy().into_owned()})",
        }

        Router::<Route> {}
    }
}
