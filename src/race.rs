use serde::{Serialize, Deserialize};

use crate::{
    basic::Stat, 
    character::char_attributes::{CreatureSize, CreatureType, Speeds}, 
    feats::feat_effect::FeatEffect, 
    proficiencies::LanguageProf
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Race {
    name: String,
    description: String,
    asi_type: RaceASI,
    // covers "X Traits" things like age, alignmnent, size, and languages
    traits: Option<Vec<String>>,
    // an internal racial choice, like a dragonborn's draconic heritage
    choice: Option<Vec<String>>,
    creature_size_choices: Vec<CreatureSize>,
    creature_type: CreatureType,
    speeds: Speeds,
    lamguages: Vec<LanguageProf>,
    abilities: Vec<RacialAbility>,
    subraces: Option<Vec<SubRace>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubRace {
    name: String,
    description: String,
    abilities: Vec<FeatEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RacialAbility {
    name: String,
    description: String,
    effect: Option<FeatEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaceASI {
    PlusTwo(Option<Stat>),
    PlusOne(Option<Stat>),
    PlusTwoPlusOne(Option<Stat>, Option<Stat>),
    TriplePlusOne(Option<Stat>, Option<Stat>, Option<Stat>),
    PlusOneToAll,
    DoublePlusTwo(Option<Stat>, Option<Stat>),
}