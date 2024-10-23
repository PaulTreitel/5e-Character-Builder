
use serde::{Serialize, Deserialize};

use crate::{
    basic::{
        AbilityScores, Alignment, Condition, CreatureSize, CreatureType, DamageResistImmune, Sense, Skill, Speeds, Stat
    }, 
    proficiencies::LanguageProf};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MonsterBase {
    name: String,
    size: CreatureSize,
    creature_type: CreatureType,
    alignment: Alignment,
    armor_class: u8,
    health: MonsterHealthStats,
    speeds: Speeds,
    scores: AbilityScores,
    senses: Vec<Sense>,
    languages: Vec<LanguageProf>,
    challenge_rating: f32,
    prof_bonus: u8,
    saving_throw_profs: Vec<Stat>,
    skill_profs: Vec<Skill>,
    dmg_resistances: Vec<DamageResistImmune>,
    dmg_immunities: Vec<DamageResistImmune>,
    condition_immunities: Vec<Condition>,
    actions: Option<Vec<String>>,
    bonus_actions: Option<Vec<String>>,
    reactions: Option<Vec<String>>,
    legendary_actions: Option<Vec<String>>,
    lair_actions: Option<Vec<String>>,
    mythic_actions: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct MonsterHealthStats {
    hit_dice: u8,
    hit_dice_mod: i32,
    avg_hit_points: u32,
}