use serde::{Deserialize, Serialize};

use crate::{basic::{ActivationTime, Condition, CreatureSize, DamageResistImmune, Sense, Skill, Speed, Stat}, proficiencies::Proficiency};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FeatEffect {
    IncreaseStat{ options: Vec<Stat>, by: u8, max: u8 },
    MaxHealthIncreaseFlat(u8),
    MaxHealthIncreaseLevel{ scalar: u8 },
    GrantProficiency(Proficiency),
    SkillAdv{ s: Skill, context: Option<String> },
    SkillDadv{ s: Skill, context: Option<String> },
    SaveAdv{ s: Stat, context: Option<String>},
    SaveDadv{ s: Stat, context: Option<String>},
    SaveAdvAgainst{ context: String },
    SaveDadvAgainst{ context: String },
    Sense(Sense),
    Speed(Speed, u32),
    SetSize(CreatureSize),
    InitiativeBonus(u8),
    Resistance(DamageResistImmune),
    ResistanceAgainst{ context: String },
    DamageImmunity(DamageResistImmune),
    ConditionImmunity(Condition),
    GrantAction{ time: ActivationTime, action_desc: String },
    Choice(Vec<String>),
    // TODO represent more things
}
