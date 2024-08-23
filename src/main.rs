use entity::Entity;

mod combat;
mod entity;


fn test1(){
    let mut player = Entity::new(
        "player test".to_string(), 
        2000, 
        200.0, 
        50.0, 
        5, 
        50, 
        entity::EntityType::Player
    );

    let mut enemy = Entity::new(
        "enemy test".to_string(), 
        1000, 
        200.0, 
        50.0, 
        6, 
        6, 
        entity::EntityType::Enemy
    );

    player.skills[0] = "fire ball".to_string();

    combat::combat(&mut player, &mut enemy);
}
fn main() {
    println!("Hello, world!");
    test1();
}
