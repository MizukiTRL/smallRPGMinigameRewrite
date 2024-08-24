use entity::Entity;

mod combat;
mod entity;
mod interface;

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
fn main() {
    println!("Hello, world!");
    test1();
}
