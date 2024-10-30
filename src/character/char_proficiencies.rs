use serde::{Deserialize, Serialize};
    
use crate::{
    basic::{Skill, Stat},
    proficiencies::{ArmorProf, LanguageProf, ProficiencyLevel, ToolProf, WeaponProf}
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharProficiencies {
    skills: Vec<(Skill, ProficiencyLevel)>,
    saves: Vec<Stat>,
    armor: Vec<ArmorProf>,
    weapons: Vec<WeaponProf>,
    languages: Vec<LanguageProf>,
    tools: Vec<(ToolProf, ProficiencyLevel)>,
    initiative: bool,
}