use crate::items::equipment::Equipment;
use crate::basic::Skill;
use crate::proficiencies::ToolLangProf;
use crate::money::Money;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Background {
    name: String,
    description: String,
    skill_profs: (Skill, Skill),
    lang_tool_profs: (ToolLangProf, ToolLangProf),
    gear: Vec<Equipment>,
    money: Money,
    character_choices: BackgoundCharacterization,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BackgoundCharacterization {
    personality: Vec<String>,
    ideals: Vec<String>,
    bonds: Vec<String>,
    flaws: Vec<String>,
    bg_choice: Option<Vec<String>>,
}

impl Background {
    pub fn new(
        name: String,
        description: String,
        skills: (Skill, Skill),
        lang_tool_profs: (ToolLangProf, ToolLangProf),
        gear: Vec<Equipment>,
        money: Money,
        character_choices: BackgoundCharacterization,
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

impl BackgoundCharacterization {
    pub fn new(
        personality: Vec<String>, 
        ideals: Vec<String>,
        bonds: Vec<String>,
        flaws: Vec<String>,
        bg_choice: Option<Vec<String>>,
    ) -> Self {
        BackgoundCharacterization { personality, ideals, bonds, flaws, bg_choice }
    }
}