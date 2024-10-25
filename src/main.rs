mod spell;
mod basic;
mod background;
mod proficiencies;
mod monster;
mod inventory;
mod feats;
mod character_attributes;


fn main() {
    println!("Hello, world!");
    feats::test_create_feats::make_feats();
}