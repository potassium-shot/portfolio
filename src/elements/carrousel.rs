use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

use crate::{prelude::*, utils::sleep};

const IMAGE_SWAP_TIME: Duration = Duration::from_secs(4);
const IMAGE_SWAP_TIME_AFTER_CLICK: Duration = Duration::from_secs(7);

#[component]
pub fn Carrousel(images: Vec<String>, size: &'static str) -> Element {
    let image_urls = images
        .iter()
        .map(|image| format!("/api/get-image?name={}&size={}", image, size));

    let image_count = images.len();

    let mut carrousel_current_image = use_signal(|| 0_usize);
    let mut image_container_size = use_signal(|| 0.0_f64);
    let images_size = use_signal(|| Rc::new(RefCell::new(vec![0.0_f64; image_count])));
    let seperator_size = use_memo(move || {
        if image_count > 1 {
            (image_container_size() - images_size().borrow().iter().copied().sum::<f64>())
                / ((image_count - 1) as f64)
        } else {
            0.0_f64
        }
    });

    let carrousel_offset = use_memo(move || {
        let i = carrousel_current_image();
        let sep = seperator_size();
        -images_size()
            .borrow()
            .iter()
            .take(i)
            .map(|size| *size + sep)
            .sum::<f64>()
    });

    let (skip_tx, skip_rx) = tokio::sync::mpsc::channel::<()>(1);
    let skip_tx = use_signal(|| skip_tx);
    let skip_rx = use_signal(|| Rc::new(Cell::new(Some(skip_rx))));

    use_future(move || async move {
        let mut skip_rx = skip_rx().take().unwrap();
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

        for i in (0..image_count).cycle().skip(1) {
            wait().await;
            carrousel_current_image.set(i);
        }
    });

    rsx! {
        div {
            id: "carrousel",
            onclick: move |_| async move { _ = skip_tx().send(()).await; },

            div {
                id: "images",
                style: "transform: translate({carrousel_offset}px);",
                onresize: move |event| {
                    if let Ok(size) = event.get_content_box_size() {
                        image_container_size.set(size.width);
                    }
                },

                for (i, image) in image_urls.into_iter().enumerate() {
                    img {
                        src: "{image}",
                        onresize: move |event| {
                            if let Ok(size) = event.get_content_box_size() {
                                images_size().borrow_mut()[i] = size.width;
                            }
                        },
                    }
                }
            }
        }
    }
}
