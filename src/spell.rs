use crate::basic::{ActivationTime, AreaOfEffect, AttackType, EffectDuration, EffectRange, Stat};
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Spell<'a> {
    pub name: &'a str,
    pub level: u8,
    pub school: SpellSchool,
    pub cast_time: ActivationTime,
    pub range: EffectRange,
    pub area: Option<AreaOfEffect>,
    pub components: SpellComponents<'a>,
    pub duration: EffectDuration,
    pub concentration: bool,
    pub save: Option<Stat>,
    pub attack: Option<AttackType>,
    pub description: &'a str,
    pub spell_lists: Vec<&'a str>,
    // TODO: How do we store (a) spell effects that act on the character sheet
    // and (b) spell upcasting benefits (which should also be displayed on
    // spells in upcast slots)?
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SpellComponents<'a> {
    verbal: bool,
    somatic: bool,
    material: bool,
    material_desc: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation
}

impl<'a> SpellComponents<'a> {
    pub fn new(v: bool, s: bool, m: bool, m_desc: Option<&'a str>) -> Self {
        SpellComponents {
            verbal: v, 
            somatic: s, 
            material: m, 
            material_desc: m_desc
        }
    }
}