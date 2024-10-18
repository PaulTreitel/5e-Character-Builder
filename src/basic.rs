use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Skill {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttackType {
    MeleeWeapon,
    RangedWeapon,
    MeleeSpell,
    RangedSpell,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stat {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffectShape {
    Square,
    Cube,
    Sphere,
    Cone,
    Cylinder,
    Line,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffectRange {
    Myself,
    Touch,
    Feet(u32),
    Miles(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money {
    platinum: i32,
    gold: i32,
    electrum: i32,
    silver: i32,
    copper: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AreaOfEffect {
    size: u32,
    shape: EffectShape,
}

impl AreaOfEffect {
    pub fn new(size: u32, shape: EffectShape) -> AreaOfEffect {
        AreaOfEffect { size: size, shape: shape }
    }

    pub fn size(self) -> u32 {
        self.size
    }

    pub fn shape(self) -> EffectShape {
        self.shape
    }
}

impl Skill {
    pub fn new(name: &str) -> Skill {
        Skill { name: name.to_owned() }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Money {
    const COPPER_PER_SILVER: i32 = 10;
    const SILVER_PER_GOLD: i32 = 10;
    const ELECTRUM_PER_GOLD: i32 = 2;
    const GOLD_PER_PLATINUM: i32 = 10;
    const SILVER_PER_ELECTRUM: i32 = 5;
    
    pub fn new(pp: i32, gp: i32, ep: i32, sp: i32, cp: i32) -> Money {
        Money { platinum: pp, gold: gp, electrum: ep, silver: sp, copper: cp }
    }

    pub fn simplify(&mut self) -> () {
        if  self.copper.abs() > Self::COPPER_PER_SILVER {
            self.silver += self.copper / Self::COPPER_PER_SILVER;
            self.copper = self.copper % Self::COPPER_PER_SILVER;
        }
        if self.silver.abs() > Self::SILVER_PER_GOLD {
            self.gold += self.silver / Self::SILVER_PER_GOLD;
            self.silver = self.silver % Self::SILVER_PER_GOLD;
        }
        if self.electrum.abs() > Self::ELECTRUM_PER_GOLD {
            self.gold += self.electrum / Self::ELECTRUM_PER_GOLD;
            self.electrum = self.electrum % Self::ELECTRUM_PER_GOLD;
        }
        self.gold += self.platinum * Self::GOLD_PER_PLATINUM;
        self.platinum = 0;
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, amt: Self) -> Self::Output {
        let mut new = Money {
            platinum: self.platinum - amt.platinum,
            gold: self.gold - amt.gold,
            electrum: self.electrum - amt.electrum,
            silver: self.silver - amt.silver,
            copper: self.copper - amt.copper,
        };
        if new.platinum < 0 {
            new.gold -= new.platinum * Self::GOLD_PER_PLATINUM;
            new.platinum = 0;
        }
        if new.gold < 0 {
            new.silver -= new.gold * Self::SILVER_PER_GOLD;
            new.gold = 0;
        }
        if new.electrum < 0 {
            new.silver -= new.electrum * Self::SILVER_PER_ELECTRUM;
            new.electrum = 0;
        }
        if new.silver < 0 {
            // Since both negative gold and negative electrum can subtract from
            // silver, we need to account for the possibility of negative
            // electrum pushing silver negative while still having positive 
            // gold. In that case we want to reduce gold first, without making 
            // gold negative.
            let mut gold_needed = new.silver.abs() / Self::SILVER_PER_GOLD;
            if gold_needed * Self::SILVER_PER_GOLD < new.silver.abs() {
                gold_needed += 1;
            }
            if new.gold >= gold_needed {
                new.silver += gold_needed * Self::SILVER_PER_GOLD;
                new.gold -= gold_needed;
            } else {
                new.silver += new.gold * Self::SILVER_PER_GOLD;
                new.gold = 0;
            }
            if new.silver < 0 {
                new.copper -= new.silver * Self::COPPER_PER_SILVER;
                new.silver = 0;
            }
        }
        if new.copper < 0 {
            new.simplify();
        }
        new
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, amt: Self) -> Self::Output {
        Money {
            platinum: self.platinum + amt.platinum,
            gold: self.gold + amt.gold,
            electrum: self.electrum + amt.electrum,
            silver: self.silver + amt.silver,
            copper: self.copper + amt.copper,
        }
    }
}