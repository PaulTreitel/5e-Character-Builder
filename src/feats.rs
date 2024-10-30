pub mod feat_effect;
pub mod test_create_feats;

use feat_effect::FeatEffect;
use serde::{Deserialize, Serialize};

use crate::{basic::Stat, proficiencies::ArmorProf};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feat {
    name: String,
    description: String,
    prereq: Option<FeatPrereq>,
    effects: Option<Vec<FeatEffect>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatPrereq {
    Race(String),
    MinStat{ s: Vec<(Stat, u8)>, stat_logical_or: bool },
    Prof(ArmorProf),
    CastASpell,
}

mod feats {
    use super::feat_effect::FeatEffect;

    use crate::feats::{Feat, FeatPrereq};

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

        pub fn effects(&self) -> &Option<Vec<FeatEffect>> {
            &self.effects
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn description(&self) -> &str {
            &self.description
        }
    }
}