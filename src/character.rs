mod char_proficiencies;
mod health;
mod char_background;
pub mod char_attributes;

use char_background::CharBackground;
use health::CharacterHealth;
use char_proficiencies::CharProficiencies;
use serde::{Deserialize, Serialize};
use crate::basic::{Condition, DamageResistImmune};
use char_attributes::{AbilityScores, Alignment, CreatureSize, CreatureType, Sense, Speeds};
use crate::{class::Class, inventory::Inventory, race::Race};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    name: String,
    player: Option<String>,
    scores: AbilityScores,
    race: Race,
    background: CharBackground,
    character_level: u8,
    experience: Option<u32>,
    main_class: Class,
    multiclasses: Option<Vec<(Class, u8)>>,
    speeds: Speeds,
    senses: Option<Vec<Sense>>,
    size: CreatureSize,
    creature_type: CreatureType,
    alignment: Option<Alignment>,
    proficiencies: CharProficiencies,
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