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

#[macro_export]
macro_rules! dx_dbg {
    () => {
        dioxus::logger::tracing::debug!("[{}:{}:{}]", std::file!(), std::line!(), std::column!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                dioxus::logger::tracing::debug!("[{}:{}:{}] {} = {:#?}",
                    std::file!(),
                    std::line!(),
                    std::column!(),
                    std::stringify!($val),
                    // The `&T: Debug` check happens here (not in the format literal desugaring)
                    // to avoid format literal related messages and suggestions.
                    &&tmp as &dyn std::fmt::Debug,
                );
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dx_dbg!($val)),+,)
    };
}
