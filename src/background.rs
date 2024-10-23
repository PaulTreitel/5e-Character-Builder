use crate::basic::Skill;
use crate::proficiencies::ToolLangProf;
use crate::money::Money;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Background<'a> {
    name: &'a str,
    description: &'a str,
    skill_profs: (Skill<'a>, Skill<'a>),
    lang_tool_profs: (ToolLangProf<'a>, ToolLangProf<'a>),
    gear: Vec<&'a str>,
    money: Money,
    character_choices: BackgoundCharacterization<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BackgoundCharacterization<'a> {
    #[serde(borrow)]
    personality: Vec<&'a str>,
    ideals: Vec<&'a str>,
    bonds: Vec<&'a str>,
    flaws: Vec<&'a str>,
    bg_choice: Option<Vec<&'a str>>,
}

impl<'a> Background<'a> {
    pub fn new(
        name: &'a str,
        description: &'a str,
        skills: (Skill<'a>, Skill<'a>),
        lang_tool_profs: (ToolLangProf<'a>, ToolLangProf<'a>),
        gear: Vec<&'a str>,
        money: Money,
        character_choices: BackgoundCharacterization<'a>,
    ) -> Self {
        Background {
            name, 
            description, 
            skill_profs: skills, 
            lang_tool_profs,
            gear, 
            money,
            character_choices
        }
    }
}

impl<'a> BackgoundCharacterization<'a> {
    pub fn new(
        personality: Vec<&'a str>, 
        ideals: Vec<&'a str>,
        bonds: Vec<&'a str>,
        flaws: Vec<&'a str>,
        bg_choice: Option<Vec<&'a str>>,
    ) -> Self {
        BackgoundCharacterization { personality, ideals, bonds, flaws, bg_choice }
    }
}