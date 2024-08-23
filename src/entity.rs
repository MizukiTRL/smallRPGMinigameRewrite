#![allow(unused)]
use super::combat::BuffType;
#[derive(Clone, Debug)]
pub struct Entity {
    name: String,
    pub stats: Stats,
    poitions: Position,
    entity_type: EntityType,
    pub skills: [String; 4],
    pub effects: Vec<BuffType>,
    pub status : Status,
}

#[derive(Clone, Debug)]
pub enum Status{
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
