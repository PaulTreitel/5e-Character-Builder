pub mod char_attributes;
mod char_background;
mod char_class;
mod char_proficiencies;
mod health;
mod recompute;
mod defenses;

use crate::{inventory::Inventory, race::Race};
use char_attributes::{AbilityScores, Alignment, CreatureSize, CreatureType, Sense, Speeds};
use char_background::CharBackground;
use char_class::CharClass;
use char_proficiencies::CharProficiencies;
use defenses::Defenses;
use health::CharacterHealth;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    name: String,
    player: String,
    pub scores: AbilityScores,
    pub base_scores: AbilityScores,
    race: Race,
    background: CharBackground,
    character_level: u8,
    experience: u32,
    main_class: CharClass,
    multiclasses: Vec<CharClass>,
    speeds: Speeds,
    senses: Vec<Sense>,
    size: CreatureSize,
    creature_type: CreatureType,
    alignment: Option<Alignment>,
    proficiencies: CharProficiencies,
    base_proficiencies: CharProficiencies,
    inventory: Inventory,
    inspiration: bool,
    pub health: CharacterHealth,
    pub defenses: Defenses,
    // TODO physical_description
    // TODO
}

pub enum CharacterError {
    LevelDownNonExistentClass,
}

mod character {
    use crate::{
        background::Background, basic::Stat, class::{Class, Classes}, inventory::Inventory, race::Race
    };

    use super::{
        char_attributes::{Alignment, CreatureSize, CreatureType, Sense, Speed, Speeds}, char_background::CharBackground, char_class::CharClass, char_proficiencies::CharProficiencies, Character, CharacterError
    };

    impl Character {
        pub fn new() -> Self {
            todo!("Write Character constructor")
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn player(&self) -> &str {
            &self.player
        }

        pub fn race(&self) -> &Race {
            &self.race
        }

        pub fn background(&self) -> &CharBackground {
            &self.background
        }

        pub fn character_level(&self) -> u8 {
            self.character_level
        }

        pub fn xp(&self) -> u32 {
            self.experience
        }

        pub fn main_class(&self) -> &CharClass {
            &self.main_class
        }

        pub fn classes(&self) -> Vec<CharClass> {
            let mut tmp = self.multiclasses.clone();
            tmp.insert(0, self.main_class.clone());
            tmp
        }

        pub fn speeds(&self) -> &Speeds {
            &self.speeds
        }

        pub fn senses(&self) -> &Vec<Sense> {
            &self.senses
        }

        pub fn size(&self) -> &CreatureSize {
            &self.size
        }

        pub fn creature_type(&self) -> &CreatureType {
            &self.creature_type
        }

        pub fn alignment(&self) -> &Option<Alignment> {
            &self.alignment
        }

        pub fn proficiencies(&self) -> &CharProficiencies {
            &self.proficiencies
        }

        pub fn inventory(&self) -> &Inventory {
            &self.inventory
        }

        pub fn has_inspiration(&self) -> bool {
            self.inspiration
        }

        pub fn pb(&self) -> u8 {
            if self.character_level <= 4 {
                2
            } else if self.character_level <= 8 {
                3
            } else if self.character_level <= 12 {
                4
            } else if self.character_level <= 16 {
                5
            } else {
                6
            }
        }

        pub fn reset_stat(&mut self, s: Stat) -> () {
            self.scores
                .set_stat(s.clone(), self.base_scores.get_stat(s));
            // self.recompute_stats();
        }

        pub fn change_background(&mut self, bg: &Background) -> () {
            self.background = CharBackground::from_background(bg);
            self.recompute();
        }

        pub fn reset_background(&mut self) -> () {
            self.background.reset_mechanics();
        }

        pub fn change_race(&mut self, race: &Race) -> () {
            self.race = race.clone();
            self.recompute();
        }

        pub fn level_up_existing_class(&mut self, class: Classes) -> () {
            todo!("Level up existing class")
        }

        pub fn level_up_new_class(&mut self, class: &Class) -> () {
            todo!("Level up with a new multiclass")
        }

        pub fn level_down(&mut self, class: Classes) -> Result<(), CharacterError> {
            todo!("Level down")
        }

        pub fn add_speed(&mut self, speed: Speed, val: u32) -> () {
            self.speeds.set_speed(speed, val);
        }

        pub fn remove_speed(&mut self, speed: Speed) -> () {
            self.speeds.set_speed(speed, 0);
            self.recompute_speeds()
        }
    }
}
