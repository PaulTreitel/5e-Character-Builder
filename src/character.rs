mod proficiencies;
mod health;

use health::CharacterHealth;
use proficiencies::CharacterProficiencies;
use serde::{Deserialize, Serialize};
use crate::{
    basic::{AbilityScores, Condition, DamageResistImmune, Speeds}, character_attributes::{Alignment, CreatureSize, CreatureType, Sense}, class::Class, inventory::Inventory
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Character {
    name: String,
    player: Option<String>,
    scores: AbilityScores,
    main_class: Class,
    multiclasses: Option<Vec<(Class, u8)>>,
    speeds: Speeds,
    senses: Vec<Sense>,
    size: CreatureSize,
    creature_type: CreatureType,
    alignment: Alignment,
    character_level: u8,
    proficiencies: CharacterProficiencies,
    inventory: Inventory,
    inspiration: bool,
    health: CharacterHealth,
    resistances: Option<Vec<DamageResistImmune>>,
    immunities: Option<Vec<DamageResistImmune>>,
    vulnerabilities: Option<Vec<DamageResistImmune>>,
    condition_immunities: Option<Vec<Condition>>,
    // TODO
}

mod character {
    
}