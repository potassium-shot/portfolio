use dioxus::fullstack::{body::Body, response::Response};

use crate::{data::PortfolioData, prelude::*};

#[get("/api/fetch-portfolio-data")]
pub async fn fetch_portfolio_data() -> ServerFnResult<PortfolioData> {
    Ok(PortfolioData::load()?)
}

#[get("/api/get-image?name&size")]
pub async fn get_image(name: String, size: String) -> ServerFnResult<Response> {
    let bytes = crate::data::image::get_image(name.as_str(), size.as_str()).ok_or_else(|| {
        ServerFnError::Args(format!(
            "Could not find image with name '{}' and size '{}'.",
            name, size
        ))
    })?;
    Ok(Response::builder()
        .header("Content-Type", "image/png")
        .body(Body::from(bytes))
        .unwrap())
}
