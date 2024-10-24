use serde::{Deserialize, Serialize};

use crate::{basic::Stat, proficiencies::ArmorProf};

use super::feat_effect::FeatEffect;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Feat {
    name: String,
    description: String,
    prereq: Option<FeatPrereq>,
    effects: Option<Vec<FeatEffect>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FeatPrereq {
    Race(String),
    MinStat{ s: Vec<(Stat, u8)>, stat_logical_or: bool },
    Prof(ArmorProf),
    CastASpell,
}

impl Feat {
    pub fn new(
        name: String, description: String, 
        prereq: Option<FeatPrereq>, 
        effects: Option<Vec<FeatEffect>>
    ) -> Self {
        Feat { name, description, prereq, effects }
    }

    pub fn add_effect(&mut self, e: FeatEffect) -> () {
        if self.effects == None {
            self.effects = Some(Vec::new());
        }
        self.effects.as_mut().unwrap().push(e);
    }

    pub fn effects(self) -> Option<Vec<FeatEffect>> {
        self.effects.clone()
    }

    pub fn name(self) -> String {
        self.name
    }

    pub fn description(self) -> String {
        self.description
    }
}