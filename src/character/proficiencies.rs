use serde::{Deserialize, Serialize};
    
use crate::{
    basic::{Skill, Stat},
    proficiencies::{ArmorProf, LanguageProf, ProficiencyLevel, ToolProf, WeaponProf}
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CharacterProficiencies {
    skills: Vec<(Skill, ProficiencyLevel)>,
    saves: Vec<Stat>,
    armor: Vec<ArmorProf>,
    weapons: Vec<WeaponProf>,
    languages: Vec<LanguageProf>,
    tools: Vec<(ToolProf, ProficiencyLevel)>,
    initiative: bool,
}