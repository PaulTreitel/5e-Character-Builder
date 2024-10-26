use serde::{Deserialize, Serialize};

use crate::basic::DieSize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CharacterHealth {
    current_max_hp: i32,
    base_max_hp: i32,
    current_hp: i32,
    temp_hp: u32,
    hit_dice: Vec<HitDice>,
    death_save_successes: u8,
    death_save_fails: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct HitDice {
    size: DieSize,
    max: u8,
    used: u8,
}

pub enum HealthError {
    ExpendMoreHitDiceThanRemain,
    ExpendNonexistentHitDie,
}

impl CharacterHealth {
    pub fn max_hp(&self) -> i32 {
        self.current_max_hp
    }

    pub fn current_hp(&self) -> i32 {
        self.current_hp
    }

    pub fn temp_hp(&self) -> u32 {
        self.temp_hp
    }

    pub fn hit_dice_remaining(&self) -> Vec<(DieSize, u8)> {
        self.hit_dice.clone()
            .iter()
            .filter(|x| x.max > x.used)
            .map(|x| (x.size.clone(), x.max - x.used))
            .collect()
    }

    pub fn change_max_hp(&mut self, amt: i32) -> () {
        if amt < 0 {
            self.current_hp += amt;
        }
        self.current_max_hp += amt;
    }

    pub fn reset_max_hp(&mut self) -> () {
        self.current_max_hp = self.base_max_hp;
    }

    pub fn restore_hit_dice(&mut self) -> () {
        todo!("restore long rest hit dice")
    }

    pub fn expend_hit_dice(&mut self, dice: &[(DieSize, u8)]) -> Result<(), HealthError> {
        todo!("expend hit dice")
    }

    pub fn take_damage(&mut self, dmg: u32) -> () {
        let mut total_taken = 0;
        if self.temp_hp > 0 {
            if self.temp_hp >= dmg {
                self.temp_hp -= dmg;
            } else {
                total_taken += self.temp_hp;
                self.temp_hp = 0;
            }
        }
        self.current_hp -= dmg as i32 - total_taken as i32;
    }

    pub fn heal(&mut self, heal:  u32) -> () {
        if self.current_hp <= 0 {
            self.current_hp = heal as i32;
        } else {
            self.current_hp += heal as i32;
        }
    }

    pub fn gain_temp_hp(&mut self, tmp_hp: u32) -> () {
        if self.temp_hp < tmp_hp {
            self.temp_hp = tmp_hp;
        }
    }

    pub fn lose_temp_hp(&mut self, amt: u32) -> () {
        if amt >= self.temp_hp {
            self.temp_hp = 0;
        } else {
            self.temp_hp -= amt;
        }
    }
}