use crate::prelude::*;

#[component]
pub fn Tag(tag: TagView) -> Element {
    let lang = use_context::<Signal<Lang>>();

    rsx! {
        span {
            id: "tag",
            style: "--l-tag-color: {tag.color.to_css()}",

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
