use itertools::Itertools;

use crate::{
    RouteKind,
    components::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger},
    prelude::*,
};

#[component]
pub fn PortfolioNavbar() -> Element {
    let route = use_route::<crate::Route>();
    let route_kind = route.kind();
    let lang = use_context::<Lang>();
    let portfolio = use_context::<PortfolioContext>();

    rsx! {
        div {
            id: "navbar",
            Navbar {
                NavbarItem {
                    id: "to-home",
                    index: 0_usize,
                    to: Route::Home {},
                    value: "home",
                    "data-current": route_kind == RouteKind::Home,
                    "Home"
                }

                NavbarNav {
                    index: 1_usize,
                    NavbarTrigger {
                        "data-current": route_kind == RouteKind::ProjectPage,
                        "Projects"
                    }
                    NavbarContent {
                        class: "navbar-content",

                        match &*portfolio.read() {
                            Some(Ok(portfolio)) => rsx! {
                                for (i, (id, project)) in portfolio.projects.iter().sorted_by(|(_, a), (_, b)| a.order.cmp(&b.order)).enumerate() {
                                    NavbarItem {
                                        index: i,
                                        to: Route::ProjectPage { project: String::from(id) },
                                        value: id,
                                        "data-current": matches!(&route, Route::ProjectPage { project } if project.as_str() == id),
                                        "{project.name.resolve(lang)}"
                                    }
                                }
                            },
                            _ => rsx! {},
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
