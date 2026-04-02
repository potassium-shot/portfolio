use dioxus::prelude::*;

mod assets;
mod home;
mod navbar;
mod prelude;

use home::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(navbar::Navbar)]
    #[route("/")]
    Home {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
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
        Router::<Route> {}
    }
}
