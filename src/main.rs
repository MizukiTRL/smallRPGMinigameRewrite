use entity::Entity;
use map::Obstacle;

mod combat;
mod entity;
mod interface;
mod map;

fn test1() {
    let mut player = Entity::new(
        "player test".to_string(),
        2000,
        200.0,
        50.0,
        5,
        5,
        entity::EntityType::Player,
    );

    let mut enemy = Entity::new(
        "enemy test".to_string(),
        100,
        200.0,
        50.0,
        6,
        6,
        entity::EntityType::Enemy,
    );

    player.skills[0] = "fire ball".to_string();
    player.skills[1] = "heal".to_string();
    player.skills[2] = "charisma". to_string();
    player.skills[3] = "splash".to_string();

    enemy.skills[0] = "heal".to_string();
    enemy.skills[1] = "fire ball".to_string();

    combat::combat(&mut player, &mut enemy);
}

fn test2(){
    let mut level1 = map::Level::new("test2".to_string(), 10, 10);
    level1.print_grid();

    let mut player = Entity::new(
        "player test".to_string(),
        2000,
        200.0,
        50.0,
        5,
        5,
        entity::EntityType::Player,
    );

    let mut enemy = Entity::new(
        "enemy test".to_string(),
        100,
        200.0,
        50.0,
        6,
        6,
        entity::EntityType::Enemy,
    );

    player.skills[0] = "fire ball".to_string();
    player.skills[1] = "heal".to_string();
    player.skills[2] = "charisma". to_string();
    player.skills[3] = "splash".to_string();

    enemy.skills[0] = "heal".to_string();
    enemy.skills[1] = "fire ball".to_string();

    let mut enemy2 = Entity::new(
        "enemy test".to_string(),
        100,
        200.0,
        50.0,
        6,
        6,
        entity::EntityType::Enemy,
    );

    enemy.skills[0] = "heal".to_string();
    enemy.skills[1] = "fire ball".to_string();

    let mut enemies = vec![];

    enemies.push(enemy);
    enemies.push(enemy2);

    let mut obstacle1 = Obstacle{x: 0, y: 1};
    let mut obstacle2 = Obstacle{x: 7, y: 8};

    let mut obstacles = vec![];

    obstacles.push(obstacle1);
    obstacles.push(obstacle2);

    map::map(&mut level1, &mut player, &mut enemies, &mut obstacles)
}
fn main() {
    println!("Hello, world!");
    test2();
}
