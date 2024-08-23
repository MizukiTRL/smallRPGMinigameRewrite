#![allow(unused)]

use super::entity::{Entity, Status};
use std::{
    clone, collections::vec_deque, io::{self, Read}, str::FromStr
};

#[derive(Clone, Debug)]
struct Skill {
    name: String,
    cost: i32,
    effects: Vec<Effect>,
}

#[derive(Clone, Debug)]
enum Effect {
    Attack(f32),
    Buff(BuffType),
}

#[derive(Clone, Debug)]
pub enum BuffType {
    AttackUp(f32),
    DamageUp(f32),
    DefenseUp(f32),
    Heal(i32),
}

impl Skill {
    pub fn new(name: String,cost: i32, effects: Vec<Effect>) -> Self {
        Skill { 
            name: name, 
            cost: cost,  
            effects:effects 
        }
    }
    pub fn new_empty() -> Self {
        Skill {
            name: "".to_string(),
            cost: 0,
            effects: vec![],
        }
    }
}

fn make_skills() -> Vec<Skill>{

    let basic = Skill::new(
        "basic".to_string(), 
        0, 
        vec![Effect::Attack(0.6)]
    );

    let fire_ball = Skill::new(
        "fire ball".to_string(),
        3,
        vec![
            Effect::Attack(1.0)
            ]
    );
    let heal = Skill::new(
        "heal".to_string(),
        2,
        vec![
            Effect::Buff(BuffType::Heal(200))
        ]
    );
    let splash = Skill::new(
        "splash".to_string(), 
        10, 
        vec![Effect::Attack(0.0)]
    );

    let skills = vec![basic, fire_ball, heal, splash];

    skills
}

pub fn combat(player: &mut Entity, enemy: &mut Entity) {
    let skill_list = make_skills();
    //turn loop
    loop {
        //menu selection loop
        loop {
            let mut input = option_input();

            match input {
                //attack
                1 => use_skill(player,"basic".to_string(),enemy, &skill_list),
                //skill
                2 => skill_menu(player, enemy, &skill_list),
                //defend
                3 => (),
                //flee
                4 => (),

                _ => println!("wrong number, please try again"),
            }
        }
    }
}

fn skill_menu(player: &mut Entity, enemy: &mut Entity, skill_list: &Vec<Skill>){
    loop {
        let mut input2 = option_input();

        match input2 {
        //skills
            1 => (),
            2 => (),
            3 => (),
            4 => (),
            //go back
            5 => break,
            _ => println!("wrong number, please try again"),
        }
    }
}

//applies the effects of skills
fn use_skill(caster: &mut Entity,skill_name: String, target: &mut Entity, skill_list: &Vec<Skill>) {
    let skill = match search_skill(skill_name, skill_list) {
        Some(a) => a,
        None => panic!("failed to find the skill"),
    };

    for effect in &skill.effects{
        match effect {
            Effect::Attack(a) => {
                calc_damage(*a, caster, target);
            },
            Effect::Buff(a) => (),
        }
    }
}

fn calc_damage(motion_value: f32, caster: &mut Entity, target: &mut Entity){
    let mut atk_up = 1.0;
    let mut dmg_up = 1.0;

    for effect in &caster.effects{
        match effect {
            BuffType::AttackUp(a) => atk_up += a,
            BuffType::DamageUp(a) => dmg_up += a,
            _ => (),
        }
    }
    let raw_dmg = ((caster.stats.atk * atk_up) * motion_value) * dmg_up;

    let mut def_up = 1.0;
    let def = target.stats.def;

    for effect in &target.effects{
        match effect {
            BuffType::DefenseUp(a) => def_up += a,
            _ => (),
        }
    }

    let total_def = def * def_up;

    let total_dmg = raw_dmg * 1.0/(1.05_f32.powf(total_def));

    println!("raw damage: {} \ntotal damage: {}", raw_dmg, total_dmg);

    target.stats.cur_hp -= total_dmg as i32;

    if target.stats.cur_hp <= 0{
        target.stats.cur_hp = 0;
        target.status = Status::Dead;
    }

}

fn search_skill(skill_name: String, skill_list: &Vec<Skill>) -> Option<&Skill>{
    for skill in skill_list {
        if skill.name == skill_name{
            return Some(skill);
        }
    }
    None
}

//takes a user input and returns it as an int
fn option_input() -> i32 {
    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("failed to read user input");

    let input_int = input_string
        .trim()
        .parse()
        .expect("failed to convert String input into i32");

    input_int
}
