use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum LocString {
    Loc { en_us: String, fr_fr: String },
    Unloc(String),
}

pub enum LocStr<'a> {
    Loc { en_us: &'a str, fr_fr: &'a str },
    Unloc(&'a str),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    EnUs,
    FrFr,
}

impl LocString {
    pub fn resolve(&self, lang: Lang) -> &str {
        match self {
            Self::Loc { en_us, fr_fr } => match lang {
                Lang::EnUs => en_us.as_str(),
                Lang::FrFr => fr_fr.as_str(),
            },
            Self::Unloc(str) => str.as_str(),
        }
    }

    pub fn as_loc_str<'a>(&'a self) -> LocStr<'a> {
        match self {
            LocString::Loc { en_us, fr_fr } => LocStr::Loc {
                en_us: en_us.as_str(),
                fr_fr: fr_fr.as_str(),
            },
            LocString::Unloc(s) => LocStr::Unloc(s.as_str()),
        }
    }
}

impl<'a> LocStr<'a> {
    pub fn resolve(&self, lang: Lang) -> &str {
        match self {
            Self::Loc { en_us, fr_fr } => match lang {
                Lang::EnUs => en_us,
                Lang::FrFr => fr_fr,
            },
            Self::Unloc(str) => str,
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

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EnUs => "en-us",
                Self::FrFr => "fr-fr",
            }
        )
    }
}
