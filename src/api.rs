use crate::{data::PortfolioData, prelude::*};

#[get("/api/fetch-portfolio-data")]
pub async fn fetch_portfolio_data() -> ServerFnResult<PortfolioData> {
    Ok(PortfolioData::load()?)
}
