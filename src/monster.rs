
use serde::{Serialize, Deserialize};

use crate::{
    basic::{Condition, DamageResistImmune, Skill, Stat}, 
    character::char_attributes::{AbilityScores, Alignment, CreatureSize, CreatureType, Sense, Speeds}, 
    proficiencies::LanguageProf
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonsterBase {
    name: String,
    size: CreatureSize,
    creature_type: CreatureType,
    alignment: Option<Alignment>,
    armor_class: u8,
    health: MonsterHealthStats,
    speeds: Speeds,
    scores: AbilityScores,
    senses: Option<Vec<Sense>>,
    languages: Option<Vec<LanguageProf>>,
    challenge_rating: f32,
    prof_bonus: u8,
    saving_throw_profs: Option<Vec<Stat>>,
    skill_profs: Option<Vec<Skill>>,
    dmg_resistances: Option<Vec<DamageResistImmune>>,
    dmg_vulnerabilities: Option<Vec<DamageResistImmune>>,
    dmg_immunities: Option<Vec<DamageResistImmune>>,
    condition_immunities: Option<Vec<Condition>>,
    actions: Option<Vec<String>>,
    bonus_actions: Option<Vec<String>>,
    reactions: Option<Vec<String>>,
    legendary_actions: Option<Vec<String>>,
    lair_actions: Option<Vec<String>>,
    mythic_actions: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MonsterHealthStats {
    hit_die_size: u8,
    hit_die_count: u8,
    hit_dice_mod: i32,
    avg_hit_points: u32,
}

impl Eq for MonsterBase {
    
}