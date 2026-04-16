use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct GlobalConfig {
    pub home_message: LocString,
    pub about_me: LocString,
    pub phone: String,
    pub email: String,
}

impl GlobalConfig {
    #[cfg(feature = "server")]
    pub fn load() -> Result<Self> {
        let str = std::fs::read_to_string(super::CONFIG_PATH.join("global_config.ron"))
            .error_code(500)?;
        ron::from_str(str.as_str()).error_code(500)
    }
}
