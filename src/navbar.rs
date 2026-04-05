use crate::{prelude::*, RouteKind};

#[component]
pub fn Navbar() -> Element {
    let route = use_route::<crate::Route>().kind();

    rsx! {
        div {
            id: "navbar",
            Link {
                id: "to-home",
                class: if route == RouteKind::Home { "current-route" },
                to: Route::Home {},
                "Home"
            }
        }

        Outlet::<Route> {}
    }
}
