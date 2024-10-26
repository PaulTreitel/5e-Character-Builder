use serde::{self, Deserialize, Serialize};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DieSize {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    DPercentile,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Skill {
    Acrobatics, 
    AnimalHandling, 
    Arcana, 
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Speeds {
    walk: u32,
    swim: u32,
    climb: u32,
    fly: u32,
    burrow: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Speed {
    Walk,
    Swim,
    Climb,
    Fly,
    Burrow,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttackType {
    MeleeWeapon,
    RangedWeapon,
    MeleeSpell,
    RangedSpell,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Stat {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AbilityScores {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Condition {
    Blinded,
    Charmed,
    Deafened,
    Exhaustion(u8),
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// There are some "Partnered Content" resistances/immunities that don't appear
// here. I'm not dealing with them specifically.
pub enum DamageResistImmune {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
    MetalWeaponBPS,
    NonMagicalBPS,
    NonMagicalBPSAtk,
    NonMagicalUnsilveredBPSAtk,
    NonMagicalNonAdamantineBPSAtk,
    NonMagicalUnsilveredNonAdamantineBPSAtk,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ActivationTime {
    Action,
    BonusAction,
    Reaction,
    Minute,
    TenMinutes,
    Hour,
    EightHours,
    Day,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffectDuration {
    Instant,
    Rounds(u32),
    Minutes(u32),
    Hours(u32),
    Days(u32),
    Dispelled,
    DispelledOrTriggered,
    Special,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffectShape {
    Square,
    Cube,
    Sphere,
    Cone,
    Cylinder,
    Line,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffectRange {
    Myself,
    Touch,
    Feet(u32),
    Miles(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AreaOfEffect {
    pub size: u32,
    pub shape: EffectShape,
}

impl fmt::Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "{:?}", self)
    }
}