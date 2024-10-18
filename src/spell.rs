use crate::basic::{ActivationTime, AreaOfEffect, AttackType, EffectDuration, EffectRange, Stat};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Spell<'a> {
    pub name: &'a str,
    pub level: u8,
    pub school: SpellSchool,
    pub cast_time: ActivationTime,
    pub range: EffectRange,
    pub area: Option<AreaOfEffect>,
    // Verbal, Somatic, Material, description of material components
    pub components: (bool, bool, bool, Option<&'a str>),
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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