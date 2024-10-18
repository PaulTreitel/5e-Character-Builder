

pub trait Proficiency {

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LanguageProf<'a> {
    Choice,
    Lang{name: &'a str},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToolProf<'a> {
    InstrumentChoice,
    Instrument{name: &'a str},
    GamingChoice,
    Game{name: &'a str},
    ArtisanChoice,
    Artisan{name: &'a str},
    Generic{name: &'a str},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArmorProf<'a> {
    Light,
    Medium,
    Heavy,
    Shields,
    Specific{name: &'a str},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponProf<'a> {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
    Specific{name: &'a str},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToolLangProf<'a> {
    Lang(LanguageProf<'a>),
    Tool(ToolProf<'a>),
}

impl Proficiency for LanguageProf<'_> {

}

impl Proficiency for ToolProf<'_> {

}

impl Proficiency for ArmorProf<'_> {

}

impl Proficiency for WeaponProf<'_> {

}