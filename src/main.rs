use basic::{AreaOfEffect, Skill, EffectRange, Money, Stat};
use spell::Spell;

mod spell;
mod basic;
mod background;
mod proficiencies;

const BASIC_SKILLS: [&str; 18] = [
    "Acrobatics", 
    "AnimalHandling", 
    "Arcana", 
    "Athletics",
    "Deception",
    "History",
    "Insight",
    "Intimidation",
    "Investigation",
    "Medicine",
    "Nature",
    "Perception",
    "Performance",
    "Persuasion",
    "Religion",
    "SleightOfHand",
    "Stealth",
    "Survival"];

fn main() {
    let mut skills = BASIC_SKILLS.map(
        |x| Skill::new(x)).to_vec();
    println!("Hello, world!");
    use_everything_spell();
    println!("{:?}", Money::new(0, 0, 0, 0, 0));
}

fn use_everything_spell() {
    let a = Spell {
        name: "t1",
        level: 1,
        school: spell::SpellSchool::Abjuration,
        cast_time: basic::ActivationTime::Action,
        area: Some(AreaOfEffect::new(30, basic::EffectShape::Cone)),
        components: (true, true, false, None),
        duration: basic::EffectDuration::Instant,
        concentration: false,
        save: Some(Stat::Charisma),
        attack: Some(basic::AttackType::MeleeSpell),
        description: "a",
        spell_lists: Vec::new(),
        range: EffectRange::Myself,
    };
    let b = Spell {
        name: "t1",
        level: 1,
        school: spell::SpellSchool::Conjuration,
        cast_time: basic::ActivationTime::BonusAction,
        area: Some(AreaOfEffect::new(30, basic::EffectShape::Line)),
        components: (true, true, false, None),
        duration: basic::EffectDuration::Rounds(1),
        concentration: false,
        save: Some(Stat::Intelligence),
        attack: Some(basic::AttackType::RangedSpell),
        description: "x",
        spell_lists: Vec::new(),
        range: EffectRange::Touch,
    };
    let c = Spell {
        name: "t1",
        level: 1,
        school: spell::SpellSchool::Evocation,
        cast_time: basic::ActivationTime::Reaction,
        area: Some(AreaOfEffect::new(30, basic::EffectShape::Square)),
        components: (true, true, false, None),
        duration: basic::EffectDuration::Minutes(1),
        concentration: false,
        save: Some(Stat::Wisdom),
        attack: Some(basic::AttackType::MeleeWeapon),
        description: "x",
        spell_lists: Vec::new(),
        range: EffectRange::Feet(60),
    };
    let d = Spell {
        name: "t1",
        level: 1,
        school: spell::SpellSchool::Enchantment,
        cast_time: basic::ActivationTime::TenMinutes,
        area: Some(AreaOfEffect::new(30, basic::EffectShape::Sphere)),
        components: (true, true, false, None),
        duration: basic::EffectDuration::Hours(1),
        concentration: false,
        save: Some(Stat::Strength),
        attack: Some(basic::AttackType::RangedWeapon),
        description: "x",
        spell_lists: Vec::new(),
        range: EffectRange::Miles(1),
    };
    let e = Spell {
        name: "t1",
        level: 1,
        school: spell::SpellSchool::Necromancy,
        cast_time: basic::ActivationTime::Hour,
        area: Some(AreaOfEffect::new(30, basic::EffectShape::Cube)),
        components: (true, true, false, None),
        duration: basic::EffectDuration::Days(1),
        concentration: false,
        save: Some(Stat::Dexterity),
        attack: Some(basic::AttackType::RangedWeapon),
        description: "x",
        spell_lists: Vec::new(),
        range: EffectRange::Myself,
    };
    let f = Spell {
        name: "t1",
        level: 1,
        school: spell::SpellSchool::Transmutation,
        cast_time: basic::ActivationTime::Minute,
        area: Some(AreaOfEffect::new(30, basic::EffectShape::Cylinder)),
        components: (true, true, false, None),
        duration: basic::EffectDuration::DispelledOrTriggered,
        concentration: false,
        save: Some(Stat::Constitution),
        attack: Some(basic::AttackType::RangedWeapon),
        description: "x",
        spell_lists: Vec::new(),
        range: EffectRange::Myself
    };
    let g = Spell {
        name: "t1",
        level: 1,
        school: spell::SpellSchool::Divination,
        cast_time: basic::ActivationTime::Day,
        area: Some(AreaOfEffect::new(30, basic::EffectShape::Cube)),
        components: (true, true, false, None),
        duration: basic::EffectDuration::Special,
        concentration: false,
        save: Some(Stat::Constitution),
        attack: Some(basic::AttackType::RangedWeapon),
        description: "x",
        spell_lists: Vec::new(),
        range: EffectRange::Myself
    };
    let h = Spell {
        name: "t1",
        level: 1,
        school: spell::SpellSchool::Illusion,
        cast_time: basic::ActivationTime::EightHours,
        area: Some(AreaOfEffect::new(30, basic::EffectShape::Cube)),
        components: (true, true, false, None),
        duration: basic::EffectDuration::Dispelled,
        concentration: false,
        save: Some(Stat::Constitution),
        attack: Some(basic::AttackType::RangedWeapon),
        description: "x",
        spell_lists: Vec::new(),
        range: EffectRange::Myself
    };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
    println!("{:?}", e);
    println!("{:?}", f);
    println!("{:?}", g);
    println!("{:?}", h);
}

