use serde::{Deserialize, Serialize};

use crate::{basic::{DamageType, DieSize}, proficiencies::WeaponProf};

use super::items::{Item, ItemRarity};



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Weapon {
    name: String,
    description: String,
    rarity: ItemRarity,
    is_magic: bool,
    req_attunement: bool,
    base_dmg_die: DieSize,
    base_dmg_type: DamageType,
    extra_dmg: Option<Vec<(DieSize, DamageType)>>,
    category: WeaponProf,
    silvered: bool,
    adamantine: bool,
    properties: Vec<WeaponProperty>,
    equipped: bool,
    // TODO anything else to represent?
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponProperty {
    Ammunition,
    Finesse,
    Heavy,
    Light,
    Loading,
    Range{base: u32, long: u32},
    Reach,
    Special,
    Thrown{base: u32, long: u32},
    TwoHanded,
    Versatile{one_hand: DieSize, two_hand: DieSize},
}

impl Weapon {

}

impl Item for Weapon {
    fn name(self) -> String {
        self.name
    }

    fn description(self) -> String {
        self.description
    }

    fn rarity(self) -> ItemRarity {
        self.rarity.to_owned()
    }

    fn is_magic(self) -> bool {
        self.is_magic
    }
}