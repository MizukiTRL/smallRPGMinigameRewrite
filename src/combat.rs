#![allow(unused)]

use super::entity::Entity;
use std::{
    io::{self, Read},
    str::FromStr,
};

struct Skill {
    name: String,
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
    pub fn new(name: String, effects: Vec<Effect>) -> Self {
        Skill { name, effects }
    }
    pub fn new_empty() -> Self {
        Skill {
            name: "".to_string(),
            effects: vec![],
        }
    }
}

pub fn make_skills() {
    let fire_ball = Skill::new(
        "fire ball".to_string(),
        vec![
            Effect::Attack(1.0)
            ]
    );
    let heal = Skill::new(
        "heal".to_string(),
        vec![
            Effect::Buff(BuffType::Heal(200))
        ]
    );
}

pub fn combat(player: &mut Entity, enemy: &mut Entity) {
    let mut turns = true;

    while turns {
        let mut selection_menu_1 = true;

        while selection_menu_1 {
            let mut input = option_input();

            match input {
                //attack
                1 => (),
                //skill
                2 => skill_menu(player, enemy),
                //defend
                3 => (),
                //flee
                4 => (),

                _ => println!("wrong number, please try again"),
            }
        }
    }
}

fn skill_menu(player: &mut Entity, enemy: &mut Entity){
    let selection_menu = true;
    while selection_menu {
        let mut input2 = option_input();

        match input2 {
        //skills
            1 => (),
            2 => (),
            3 => (),
            4 => (),
            //go back
            5 => (),
            _ => println!("wrong number, please try again"),
        }
    }
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
