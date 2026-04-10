#[macro_export]
macro_rules! css_assets {
    [$($name: expr),*,] => {
        &[
            $(
                dioxus::prelude::asset!(concat!("/assets/", $name, ".css"))
            ),*
        ]
    };
}
