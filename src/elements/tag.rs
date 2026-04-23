use std::collections::HashSet;

use crate::prelude::*;

#[component]
pub fn Tag(
    tag_id: String,
    tag: TagView,
    onclick: Option<EventHandler<MouseEvent>>,
    active_list: Option<ReadSignal<HashSet<String>>>,
) -> Element {
    let interactable = onclick.is_some();

    let lang = use_context::<Signal<Lang>>();

    rsx! {
        span {
            id: "tag",
            style: "--l-tag-color: {tag.color.to_css()}",
            class: if interactable { "interactable" } else { "" },
            onclick: move |e| if let Some(onclick) = onclick { onclick.call(e) },
            "data-active": active_list.map(|list| list().contains(&tag_id)).unwrap_or(false),

            span {
                id: "hashtag",
                "#"
            }
            span {
                id: "name",
                "{tag.name.resolve(lang())}"
            }
        }
    }
}
