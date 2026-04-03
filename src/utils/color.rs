#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum PortfolioColor {
    Theme(String),
    Rgb(u8, u8, u8),
    Hex(String),
}
