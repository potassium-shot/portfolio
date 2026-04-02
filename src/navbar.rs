use crate::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                id: "to-home",
                to: Route::Home {},
                "Home"
            }
        }

        Outlet::<Route> {}
    }
}
