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
    let mut lang = use_context::<Signal<Lang>>();
    let portfolio = use_context::<PortfolioContext>();

    rsx! {
        div {
            id: "navbar",
            Navbar {
                div {
                    id: "navbar-items",

                    NavbarItem {
                        id: "to-home",
                        index: 0_usize,
                        to: Route::Home,
                        value: "home",
                        "data-current": route_kind == RouteKind::Home,
                        "{crate::home::TITLE.resolve(lang())}"
                    }

                    NavbarNav {
                        index: 1_usize,
                        NavbarTrigger {
                            "data-current": route_kind == RouteKind::ProjectPage,
                            "{crate::project_page::TITLE.resolve(lang())}"
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
                                            "{project.name.resolve(lang())}"
                                        }
                                    }
                                },
                                _ => rsx! {},
                            }
                        }
                    }

                    NavbarItem {
                        index: 2_usize,
                        to: Route::AboutMe,
                        value: "about-me",
                        "data-current": route_kind == RouteKind::AboutMe,
                        "{crate::about_me::TITLE.resolve(lang())}"
                    }
                }

                div { flex_grow: 1 }

                match &*portfolio.read() {
                    Some(Ok(portfolio)) => rsx! {
                        span {
                            id: "contact",
                            "Contact me: {portfolio.global_config.phone} | {portfolio.global_config.email}"
                        }
                    },
                    _ => rsx! {},
                }

                button {
                    id: "lang-selector",
                    onclick: move |_| {
                        lang.set(match lang() {
                            Lang::EnUs => Lang::FrFr,
                            Lang::FrFr => Lang::EnUs,
                        });
                    },
                    "data-lang": "{lang()}",

                    img {
                        src: {match lang() {
                            Lang::EnUs => crate::assets::FR_FR,
                            Lang::FrFr => crate::assets::EN_US,
                        }}
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
