use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LanguageProf<'a> {
    Choice,
    Lang{name: &'a str},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ToolProf<'a> {
    InstrumentChoice,
    Instrument{name: &'a str},
    GamingChoice,
    Game{name: &'a str},
    ArtisanChoice,
    Artisan{name: &'a str},
    Generic{name: &'a str},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArmorProf<'a> {
    Light,
    Medium,
    Heavy,
    Shields,
    Specific{name: &'a str},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponProf<'a> {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
    Specific{name: &'a str},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ToolLangProf<'a> {
    #[serde(borrow)]
    Lang(LanguageProf<'a>),
    Tool(ToolProf<'a>),
}
