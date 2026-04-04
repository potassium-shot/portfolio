#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum PortfolioColor {
    Theme(String),
    Rgb(u8, u8, u8),
    Hex(String),
}

impl PortfolioColor {
    pub fn to_css(&self) -> String {
        match self {
            PortfolioColor::Theme(name) => format!("var(--{}, var(--white))", name),
            PortfolioColor::Rgb(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
            PortfolioColor::Hex(hex) => format!("#{}", hex),
        }
    }
}
