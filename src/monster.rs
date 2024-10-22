
use serde::{Serialize, Deserialize};

use crate::{
    basic::{
        AbilityScores, Alignment, Condition, CreatureSize, CreatureType, DamageResistImmune, Sense, Skill, Speeds, Stat
    }, 
    proficiencies::LanguageProf};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MonsterBase<'a> {
    name: &'a str,
    size: CreatureSize,
    creature_type: CreatureType<'a>,
    alignment: Alignment,
    armor_class: u8,
    health: MonsterHealthStats,
    speeds: Speeds,
    scores: AbilityScores,
    senses: Vec<Sense<'a>>,
    languages: Vec<LanguageProf<'a>>,
    challenge_rating: f32,
    prof_bonus: u8,
    saving_throw_profs: Vec<Stat>,
    skill_profs: Vec<Skill<'a>>,
    dmg_resistances: Vec<DamageResistImmune<'a>>,
    dmg_immunities: Vec<DamageResistImmune<'a>>,
    condition_immunities: Vec<Condition<'a>>,
    actions: Option<Vec<&'a str>>,
    bonus_actions: Option<Vec<&'a str>>,
    reactions: Option<Vec<&'a str>>,
    legendary_actions: Option<Vec<&'a str>>,
    lair_actions: Option<Vec<&'a str>>,
    mythic_actions: Option<Vec<&'a str>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct MonsterHealthStats {
    hit_dice: u8,
    hit_dice_mod: i32,
    avg_hit_points: u32,
}