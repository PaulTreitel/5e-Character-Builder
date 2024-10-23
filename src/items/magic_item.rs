use serde::{Deserialize, Serialize};

use crate::money::Money;

use super::items::{Equippable, Item, ItemRarity};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MagicItem {
    name: String,
    description: String,
    rarity: ItemRarity,
    equipped: Equippable,
    weight: Option<u32>,
    cost: Option<Money>,
    req_attunement: bool,
    consumable: bool,
    // TODO what else to represent?
}

impl Item for MagicItem {
    fn name(self) -> String {
        self.name
    }

    fn description(self) -> String {
        self.description
    }

    fn rarity(self) -> ItemRarity {
        self.rarity
    }

    fn is_magic(self) -> bool {
        true
    }
}