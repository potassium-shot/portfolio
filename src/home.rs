use crate::prelude::*;

#[component]
pub fn Home() -> Element {
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
            button {
                id:" test",
                "This button buttons"
            }
        }
    }
}
