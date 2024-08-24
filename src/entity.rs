#![allow(unused)]
use crate::combat::StatusEffect;

use super::combat::BuffType;
#[derive(Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub stats: Stats,
    pub poitions: Position,
    pub entity_type: EntityType,
    pub skills: [String; 4],
    pub effects: Vec<StatusEffect>,
    pub status: Status,
}

#[derive(Clone, Debug)]
pub enum Status {
    Dead,
    Alive,
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub max_hp: i32,
    pub cur_hp: i32,
    pub atk: f32,
    pub def: f32,
}

#[derive(Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub enum EntityType {
    Player,
    Enemy,
}

impl Entity {
    pub fn new(
        name: String,
        max_hp: i32,
        atk: f32,
        def: f32,
        x: i32,
        y: i32,
        entity_type: EntityType,
    ) -> Self {
        Entity {
            name: name,
            stats: Stats {
                max_hp: max_hp,
                cur_hp: max_hp,
                atk: atk,
                def: def,
            },
            poitions: Position { x: x, y: y },
            entity_type: entity_type,
            skills: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            effects: vec![],
            status: Status::Alive,
        }
    }

    pub fn get_x(&self) -> usize{
        self.poitions.x as usize
    }
    pub fn get_y(&self) -> usize{
        self.poitions.y as usize
    }

    pub fn move_up(&mut self){
        self.poitions.y -= 1;
    }

    pub fn move_left(&mut self){
        self.poitions.x -= 1;;
    }

    pub fn move_down(&mut self){
        self.poitions.y += 1
    }

    pub fn move_right(&mut self){
        self.poitions.x += 1
    }
}
