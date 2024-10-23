use serde::{Deserialize, Serialize};

use crate::proficiencies::ArmorProf;

use super::items::{Item, ItemRarity};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Armor {
    name: String,
    description: String,
    shield_or_armor: ArmorType,
    weight: u32,
    is_magic: bool,
    req_attunement: bool,
    rarity: ItemRarity,
    category: ArmorProf,
    equipped: bool,
    // TODO anything else to represent?
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum ArmorType {
    Shield,
    Armor{base: u8, dex_cap: Option<u8>, str_req: Option<u8>, stealth_dadv: bool}
}

impl Item for Armor {
    fn name(self) -> String {
        self.name
    }

    fn description(self) -> String {
        self.description
    }

    fn rarity(self) -> super::items::ItemRarity {
        self.rarity
    }

    fn is_magic(self) -> bool {
        self.is_magic
    }
}