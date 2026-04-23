#![allow(dead_code)]

use std::time::Duration;

pub mod color;
pub mod error;
pub mod ext;
pub mod loc_string;
pub mod macros;

#[cfg(feature = "web")]
pub async fn sleep(dur: Duration) {
    gloo_timers::future::sleep(dur).await;
}

#[cfg(not(feature = "web"))]
pub async fn sleep(dur: Duration) {
    tokio::time::sleep(dur).await;
}
