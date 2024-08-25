#![allow(unused)]

use std::{io::{self, Empty}, slice::ChunksExact};

use crate::{combat, entity::Entity};

#[derive(Clone, Debug)]
pub struct Level{
    name: String,
    grid: Grid,
}

#[derive(Clone, Debug)]
pub struct Grid{
    grid: Vec<Vec<Tile>>,
    height: i32,
    width: i32,
}

#[derive(Clone, Debug, PartialEq)]
enum Tile{
    Empty,
    Player,
    Enemy,
    Obstacle,
}

pub struct Obstacle{
    pub x: i32,
    pub y: i32,
}


impl Level{
    pub fn new(name: String, width: i32, height: i32) -> Self{
        
        let mut grid: Vec<Vec<Tile>> = vec![];

        for _i in 0..height{
            let mut row: Vec<Tile> = vec![];
            for _j in 0..width{
                row.push(Tile::Empty);
            }
            grid.push(row);
        }
        
        Level{name, grid: Grid{grid, height, width}}
    }
    
    pub fn print_grid(&self){
        for rows in &self.grid.grid{
            for tiles in rows{
                match tiles {
                    Tile::Empty => print!(". "),
                    Tile::Enemy => print!("E "),
                    Tile::Obstacle => print!("█ "),
                    Tile::Player => print!("p "),
                }
            }
            println!();
        }
    }


    pub fn update_grid(&mut self, player: &Entity, enemies: &Vec<Entity>, obstacles: &Vec<Obstacle>){
        
        for rows in &mut self.grid.grid{
            for tile in rows {
                *tile = Tile::Empty;
            }
        }

        self.grid.grid[player.get_y()][player.get_x()] = Tile::Player;

        for enemy in enemies{
            if self.grid.grid[enemy.get_y()][enemy.get_y()] == Tile::Empty{
                self.grid.grid[enemy.get_y()][enemy.get_x()] = Tile::Enemy
            }
        }

        for obstacle in obstacles{
            if self.grid.grid[obstacle.y as usize][obstacle.x as usize] == Tile::Empty{
                self.grid.grid[obstacle.y as usize][obstacle.x as usize] = Tile::Obstacle
            }
        }
        
    }
}

pub fn map(level: &mut Level, player: &mut Entity, enemies: &mut Vec<Entity>, obstacles: &mut Vec<Obstacle>){

    loop {
        level.update_grid(player, enemies, obstacles);
        level.print_grid();
        movement(level, player, enemies, obstacles);
    }
}

pub fn movement(level: &mut Level, player: &mut Entity, enemies: &mut Vec<Entity>, obstacles: &mut Vec<Obstacle>){
    loop {
        let input = option_input();
        let mut alive = true;
        match input {
            'w' => {
                if player.get_y() > 0{
                    player.move_up();
                    if check_colision_obstacle(player, obstacles){
                        player.move_down();
                    }
                }
                let enemy =check_colision_enemy(player, enemies);
                match enemy {
                    Some(a) => alive = combat::combat(player, a),
                    None => break,
                }
            },
            'a' => {
                if player.get_x() > 0{
                    player.move_left();
                    if check_colision_obstacle(player, obstacles){
                        player.move_right();
                    }
                }
                let enemy =check_colision_enemy(player, enemies);
                match enemy {
                    Some(a) => alive = combat::combat(player, a),
                    None => break,
                }

            },
            's' => {
                if (player.get_y() as i32) < (level.grid.height -1){
                    player.move_down();
                    if check_colision_obstacle(player, obstacles){
                        player.move_up();
                    }
                }
                let enemy =check_colision_enemy(player, enemies);
                match enemy {
                    Some(a) => alive = combat::combat(player, a),
                    None => break,
                }
            },
            'd' => {
                if (player.get_x() as i32) < (level.grid.width -1){
                    player.move_right();
                    if check_colision_obstacle(player, obstacles){
                        player.move_left();
                    }
                }
                let enemy =check_colision_enemy(player, enemies);
                match enemy {
                    Some(a) => alive = combat::combat(player, a),
                    None => break,
                }
            },
            _ => println!("wrong input"),
        }
    }
}

pub fn check_colision_obstacle(player: &mut Entity, obstacles: &Vec<Obstacle>)-> bool{
    let mut  ans = false;
    for obstacle in obstacles{
        let check1 = (player.get_x() as i32) == obstacle.x;
        let check2 = (player.get_y() as i32) == obstacle.y;

        ans = (check1 && check2);
    }
    ans
}

pub fn check_colision_enemy<'a>(player: &Entity, enemies: &'a mut Vec<Entity>) -> Option<&'a mut Entity>{
    let mut  ans = false;
    for enemy in enemies{
        let check1 = player.get_x() == enemy.get_x();
        let check2 = player.get_y() == enemy.get_x();

        ans = (check1 && check2);

        if ans{
            return Some(enemy);
        }
    }

    None

}

fn option_input() -> char {
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