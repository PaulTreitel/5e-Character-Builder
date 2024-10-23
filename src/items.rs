pub mod equipment;
pub mod weapon;
pub mod magic_item;
pub mod armor;

mod items {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub enum ItemRarity {
        Common,
        Uncommon,
        Rare,
        VeryRare,
        Legendary,
        Artifact,
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub enum Equippable {
        Unequippable,
        Unequipped,
        Equipped,
    }

    pub trait Item {
        fn name(self) -> String;
        fn description(self) -> String;
        fn rarity(self) -> ItemRarity;
        fn is_magic(self) -> bool;
    }
}