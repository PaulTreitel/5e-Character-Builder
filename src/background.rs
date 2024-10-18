use crate::basic::{Skill, Money};
use crate::proficiencies::ToolLangProf;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Background<'a> {
    name: &'a str,
    description: &'a str,
    skill_profs: (&'a Skill, &'a Skill),
    lang_tool_profs: (&'a ToolLangProf<'a>, &'a ToolLangProf<'a>),
    gear: Vec<&'a str>,
    money: Money,
    personalities: Option<Vec<&'a str>>,
    ideals: Option<Vec<&'a str>>,
    bonds: Option<Vec<&'a str>>,
    flaws: Option<Vec<&'a str>>,
    character_choice: Option<Vec<&'a str>>
}

impl<'a> Background<'a> {
    pub fn new(
        name: &'a str,
        description: &'a str,
        skills: (&'a Skill, &'a Skill),
        lang_tool_profs: (&'a ToolLangProf, &'a ToolLangProf),
        gear: Vec<&'a str>,
        money: Money,
        personalities: Option<Vec<&'a str>>,
        ideals: Option<Vec<&'a str>>,
        bonds: Option<Vec<&'a str>>,
        flaws: Option<Vec<&'a str>>,
        character_choice: Option<Vec<&'a str>>,
    ) -> Self {
        Background {
            name, 
            description, 
            skill_profs: skills, 
            lang_tool_profs, 
            gear, 
            money, 
            personalities, 
            ideals, 
            bonds, 
            flaws, 
            character_choice
        }
    }

    pub fn set_personalities(&mut self, p: Vec<&'a str>) {
        self.personalities = Some(p)
    }
    
    pub fn set_ideals(&mut self, i: Vec<&'a str>) -> () {
        self.ideals = Some(i);
    }

    pub fn set_bonds(&mut self, b: Vec<&'a str>) -> () {
        self.bonds = Some(b);
    }

    pub fn set_flaws(&mut self, f: Vec<&'a str>) -> () {
        self.flaws = Some(f);
    }

    pub fn set_choices(&mut self, c: Vec<&'a str>) -> () {
        self.character_choice = Some(c);
    }
}