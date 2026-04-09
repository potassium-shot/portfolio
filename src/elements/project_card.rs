use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

use crate::{elements::tag::Tag, prelude::*, utils::sleep};

const IMAGE_SWAP_TIME: Duration = Duration::from_secs(4);
const IMAGE_SWAP_TIME_AFTER_CLICK: Duration = Duration::from_secs(7);

#[component]
pub fn ProjectCard(project: ProjectView) -> Element {
    let lang = use_context::<Lang>();
    let portfolio = use_context::<PortfolioContext>();

    let tags = project
        .tags
        .iter()
        .flat_map(|tag| portfolio.unwrap().unwrap().find_tag(tag.as_str()));

    let images = project
        .images
        .iter()
        .map(|image| format!("api/get-image?name={}&size=thumbnail", image));

    let image_count = project.images.len();

    let mut caroussel_offset = use_signal(|| 0.0_f32);

    let mut image_container_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let image_elements = use_signal(|| Rc::new(RefCell::new(Vec::<Rc<MountedData>>::new())));

    let (skip_tx, skip_rx) = tokio::sync::mpsc::channel::<()>(1);
    let skip_tx = use_signal(|| skip_tx);
    let skip_rx = use_signal(|| Rc::new(Cell::new(Some(skip_rx))));

    use_future(move || async move {
        let mut skip_rx = skip_rx().take().unwrap();

        loop {
            sleep(Duration::from_millis(100)).await;

            if let Ok(elements) = image_elements().try_borrow()
                && elements.len() == image_count
                && image_container_element().is_some()
            {
                break;
            }
        }

        let full_width = image_container_element()
            .unwrap()
            .get_client_rect()
            .await
            .unwrap()
            .width();

        let image_elements = image_elements()
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        let mut image_widths = Vec::new();

        for image_element in image_elements {
            image_widths.push(
                image_element
                    .get_client_rect()
                    .await
                    .map(|rect| rect.width())
                    .unwrap_or(0.0),
            );
        }

        let total_width = image_widths.iter().copied().sum::<f64>();
        let sep = (full_width - total_width) / ((image_widths.len() - 1) as f64);

        image_widths.pop();

        let mut last_swap_was_manual = false;

        let mut wait = async || {
            tokio::select! {
                _ = sleep(if last_swap_was_manual { IMAGE_SWAP_TIME_AFTER_CLICK } else { IMAGE_SWAP_TIME }) => {
                    last_swap_was_manual = false;
                }
                _ = skip_rx.recv() => {
                    last_swap_was_manual = true;
                }
            }
        };

        loop {
            for width in image_widths.iter().copied() {
                wait().await;
                caroussel_offset.set(caroussel_offset() + (width + sep) as f32);
            }

            wait().await;
            caroussel_offset.set(0.0);
        }
    });

    rsx! {
        div {
            id: "project-card",

            div {
                id: "text",

                h3 { "{project.name.resolve(lang)}" }

                p {
                    for tag in tags {
                        Tag {
                            tag: tag,
                        }

                        span { " " }
                    }

                    span {
                        id: "short-description",
                        "{project.short_description.resolve(lang)}"
                    }
                }
            }

            div {
                id: "caroussel",
                onclick: move |_| async move { _ = skip_tx().send(()).await; },

                div {
                    id: "images",
                    style: "transform: translate(-{caroussel_offset}px, 0px);",
                    onmount: move |element| image_container_element.set(Some(element.data())),

                    for image in images {
                        img {
                            src: "{image}",
                            onmount: move |element| image_elements().borrow_mut().push(element.data()),
                        }
                    }
                }
            }
        }
    }
}
