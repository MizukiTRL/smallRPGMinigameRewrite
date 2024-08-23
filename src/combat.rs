#![allow(unused)]

use super::entity::Entity;
use std::{
    collections::vec_deque, io::{self, Read}, str::FromStr
};

struct Skill {
    name: String,
    cost: i32,
    effects: Vec<Effect>,
}

enum Effect {
    Attack(f32),
    Buff(BuffType),
}

enum BuffType {
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
                1 => use_skill("basic".to_string(),enemy, &skill_list),
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
fn use_skill(skill_name: String, target: &mut Entity, skill_list: &Vec<Skill>) {
    let skill = match search_skill(skill_name, skill_list) {
        Some(a) => a,
        None => panic!("failed to find the skill"),
    };

    for effect in &skill.effects{
        match effect {
            Effect::Attack(a) => (),
            Effect::Buff(a) => (),
        }
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
