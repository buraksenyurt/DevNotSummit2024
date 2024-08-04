use crate::components::*;
use crate::entities::*;
use crate::systems::*;

mod components;
mod entities;
mod systems;

fn main() {
    let towers = vec![Tower {
        position: Position { x: 1.0, y: 1.0 },
        health: Health {
            current: 1000,
            max: 1000,
        },
        range: Range { value: 5.0 },
        damage: Damage { amount: 10 },
    }];

    let mut enemies = vec![
        Enemy {
            position: Position { x: 5.0, y: 5.0 },
            health: Health {
                current: 50,
                max: 50,
            },
            velocity: Velocity {
                speed: 1.0,
                direction: (-1.0, -1.0),
            },
        },
        Enemy {
            position: Position { x: 3.0, y: 7.0 },
            health: Health {
                current: 50,
                max: 50,
            },
            velocity: Velocity {
                speed: 1.0,
                direction: (-1.0, -1.0),
            },
        },
    ];

    let mut bullets: Vec<Bullet> = Vec::new();

    println!("Initial state:");
    println!("Towers: {:#?}", towers);
    println!("Enemies: {:#?}", enemies);
    println!("Bullets: {:#?}", bullets);
    println!();

    println!("Running movement system...");
    movement_system(&mut enemies, &mut bullets);
    println!();

    println!("Running attack system...");
    attack_system(&towers, &mut enemies, &mut bullets);
    println!();

    println!("Running health system...");
    health_system(&mut enemies);
    println!();

    println!("Final state:");
    println!("Towers: {:#?}", towers);
    println!("Enemies: {:#?}", enemies);
    println!("Bullets: {:#?}", bullets);
}
