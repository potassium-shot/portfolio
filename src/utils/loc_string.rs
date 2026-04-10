use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum LocString {
    Loc { en_us: String, fr_fr: String },
    Unloc(String),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    EnUs,
    FrFr,
}

impl LocString {
    pub fn resolve(&self, lang: Lang) -> &str {
        match self {
            LocString::Loc { en_us, fr_fr } => match lang {
                Lang::EnUs => en_us.as_str(),
                Lang::FrFr => fr_fr.as_str(),
            },
            LocString::Unloc(str) => str.as_str(),
        }
    }
}

impl Default for LocString {
    fn default() -> Self {
        Self::Unloc(String::default())
    }
}

impl FromStr for Lang {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Self::EnUs),
            "fr" => Ok(Self::FrFr),
            _ => Err(()),
        }
    }
}
