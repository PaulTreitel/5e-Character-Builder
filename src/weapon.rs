use serde::{Deserialize, Serialize};

use crate::basic::{DamageType, DieSize, Item, Rarity};



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Weapon<'a> {
    name: &'a str,
    description: &'a str,
    rarity: Rarity,
    is_magic: bool,
    base_dmg_die: DieSize,
    base_dmg_type: DamageType,
    extra_dmg: Option<Vec<(DieSize, DamageType)>>,
    category: WeaponType,
    silvered: bool,
    adamantine: bool,
    properties: Vec<WeaponProperty>,

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponType {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
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

impl<'a> Weapon<'a> {

}

impl<'a> Item<'_> for Weapon<'_> {
    fn name(self) -> String {
        self.name.to_string()
    }

    fn description(self) -> String {
        self.description.to_string()
    }

    fn rarity(self) -> Rarity {
        self.rarity.to_owned()
    }

    fn is_magic(self) -> bool {
        self.is_magic
    }
}