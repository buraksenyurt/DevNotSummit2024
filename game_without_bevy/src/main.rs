use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Entity {
    pub x: f32,
    pub health: i32,
    pub speed: f32,
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut player = Entity {
        x: rng.gen_range(0.0..10.0),
        health: 100,
        speed: 1.5,
    };

    let mut enemy = Entity {
        x: rng.gen_range(5.0..10.0),
        health: 50,
        speed: 1.0,
    };

    let goal = Entity {
        x: 20.0,
        health: 0,
        speed: 0.0,
    };

    println!("Initial positions:");
    println!("Player at: {:.1}, Health: {}", player.x, player.health);
    println!("Enemy at: {:.1}, Health: {}", enemy.x, enemy.health);
    println!("Goal at: {:.1}", goal.x);
    println!("\n\n");

    while player.x < goal.x && player.health > 0 && enemy.health > 0 {
        player.x += player.speed * 0.5;
        println!("Player at: {:.1}, Health: {}", player.x, player.health);

        let enemy_direction: f32 = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        enemy.x += enemy.speed * 0.5 * enemy_direction;
        println!("Enemy at: {:.1}, Health: {}", enemy.x, enemy.health);

        if (player.x - enemy.x).abs() < 0.5 {
            println!("Collision detected with Enemy!");
            player.health -= 10;
            enemy.health -= 10;
        }

        if (player.x - goal.x).abs() < 0.5 {
            println!("You reached the goal!");
            break;
        }

        sleep(Duration::from_millis(100));
    }

    if player.health <= 0 {
        println!("Player died.");
    } else if enemy.health <= 0 {
        println!("Enemy died.");
    }
}
