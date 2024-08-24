use crate::combat::{BuffType, Skill};
use crate::entity::Entity;

pub fn battle_interface(player: &Entity, enemy: &Entity) {
    println!("enemy HP: {}/{}", enemy.stats.cur_hp, enemy.stats.max_hp);
    println!("");
    println!("player HP: {}/{}", player.stats.cur_hp, player.stats.max_hp);
    println!("");
    println!("1- attack     2- skills    3- defend      4- flee")
}

pub fn skill_selection_interface(caster: &Entity) {
    for (i, skill) in caster.skills.iter().enumerate() {
        println!("{}- {}", i + 1, skill);
    }
}

pub fn damage_promp(caster: &Entity, target: &Entity, damage: i32) {
    println!(
        "{} has dealt {} damage to {}",
        caster.name, damage, target.name
    );
}

pub fn heal_promp(caster: &Entity, healing: i32) {
    println!("{} has healed by {}", caster.name, healing);
}

pub fn buff_promp(caster: &Entity, effect: BuffType) {
    let mut effect_type = String::new();
    let mut buff_percent = 0;

    match effect {
        BuffType::AttackUp(a) => {
            effect_type = String::from("Attack");
            buff_percent = (a * 100.0) as i32;
        }
        BuffType::DamageUp(a) => {
            effect_type = String::from("Damage");
            buff_percent = (a * 100.0) as i32;
        }
        BuffType::DefenseUp(a) => {
            effect_type = String::from("Defense");
            buff_percent = (a * 100.0) as i32;
        }
    }

    println!(
        "{} has increased their {} by {}%",
        caster.name, effect_type, buff_percent
    );
}
