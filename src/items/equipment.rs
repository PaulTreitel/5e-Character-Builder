use serde::{Deserialize, Serialize};

use crate::money::Money;

use super::items::{Item, ItemRarity};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Equipment {
    name: String,
    description: String,
    weight: Option<u32>,
    cost: Option<Money>,
    consumable: bool,
    // TODO what else to represent?
}

pub struct EquipmentPack {
    name: String,
    description: String,
    weight: u32,
    cost: Money,
    contents: Vec<Equipment>,
}

impl Item for Equipment {
    fn name(self) -> String {
        self.name
    }

    fn description(self) -> String {
        self.description
    }

    fn rarity(self) -> ItemRarity {
        ItemRarity::Common
    }

    fn is_magic(self) -> bool {
        false
    }
}

impl Item for EquipmentPack {
    fn name(self) -> String {
        self.name
    }

    fn description(self) -> String {
        self.description
    }

    fn rarity(self) -> ItemRarity {
        ItemRarity::Common
    }

    fn is_magic(self) -> bool {
        false
    }
}