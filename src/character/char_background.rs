use crate::basic::Skill;
use crate::proficiencies::ToolLangProf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharBackground {
    name: String,
    description: String,
    skill_profs: (Skill, Skill),
    tool_lang_profs: (ToolLangProf, ToolLangProf),
    personality: Vec<String>,
    ideals: Vec<String>,
    bonds: Vec<String>,
    flaws: Vec<String>,
    bg_choice: Vec<String>,
}

pub enum CharBGError {
    SwappingNonExistentSkill,
    SwappingNonExistentProficiency,
    CharacterTraitOutOfBounds,
}

impl CharBackground {
    pub fn switch_skill_prof(&mut self, old: Skill, new: Skill) -> Result<(), CharBGError> {
        if self.skill_profs.0 == old {
            self.skill_profs = (new, self.skill_profs.1.clone());
            Ok(())
        } else if self.skill_profs.1 == old {
            self.skill_profs = (self.skill_profs.0.clone(), new);
            Ok(())
        } else {
            Err(CharBGError::SwappingNonExistentSkill)
        }
    }

    pub fn switch_tool_lang_prof(
        &mut self, 
        old: ToolLangProf, 
        new: ToolLangProf
    ) -> Result<(), CharBGError> {
        if self.tool_lang_profs.0 == old {
            self.tool_lang_profs = (new, self.tool_lang_profs.1.clone());
            Ok(())
        } else if self.tool_lang_profs.1 == old {
            self.tool_lang_profs = (self.tool_lang_profs.0.clone(), new);
            Ok(())
        } else {
            Err(CharBGError::SwappingNonExistentProficiency)
        }
    }

    pub fn add_personality_trait(&mut self, p: &str) -> () {
        self.personality.push(p.to_string());
    }

    pub fn add_ideal(&mut self, i: &str) -> () {
        self.ideals.push(i.to_string());
    }

    pub fn add_bond(&mut self, b: &str) -> () {
        self.bonds.push(b.to_string());
    }

    pub fn add_flaw(&mut self, f: &str) -> () {
        self.flaws.push(f.to_string());
    }

    pub fn add_bg_choice(&mut self, c: &str) -> () {
        self.bg_choice.push(c.to_string());
    }

    pub fn remove_personality_trait(&mut self, i: usize) -> Result<(), CharBGError> {
        if i < self.personality.len() {
            self.personality.remove(i);
            Ok(())
        } else {
            Err(CharBGError::CharacterTraitOutOfBounds)
        }
    }

    pub fn remove_ideal(&mut self, i: usize) -> Result<(), CharBGError> {
        if i < self.ideals.len() {
            self.ideals.remove(i);
            Ok(())
        } else {
            Err(CharBGError::CharacterTraitOutOfBounds)
        }
    }

    pub fn remove_bond(&mut self, i: usize) -> Result<(), CharBGError> {
        if i < self.bonds.len() {
            self.bonds.remove(i);
            Ok(())
        } else {
            Err(CharBGError::CharacterTraitOutOfBounds)
        }
    }

    pub fn remove_flaw(&mut self, i: usize) -> Result<(), CharBGError> {
        if i < self.flaws.len() {
            self.flaws.remove(i);
            Ok(())
        } else {
            Err(CharBGError::CharacterTraitOutOfBounds)
        }
    }

    pub fn remove_bg_choice(&mut self, i: usize) -> Result<(), CharBGError> {
        if i < self.bg_choice.len() {
            self.bg_choice.remove(i);
            Ok(())
        } else {
            Err(CharBGError::CharacterTraitOutOfBounds)
        }
    }
}