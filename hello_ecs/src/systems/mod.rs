use crate::components::*;
use crate::entities::*;

pub fn movement_system(enemies: &mut [Enemy], bullets: &mut [Bullet]) {
    for enemy in enemies.iter_mut() {
        enemy.position.x += enemy.velocity.speed * enemy.velocity.direction.0;
        enemy.position.y += enemy.velocity.speed * enemy.velocity.direction.1;
        println!(
            "Enemy moved to position ({}, {})",
            enemy.position.x, enemy.position.y
        );
    }

    for bullet in bullets.iter_mut() {
        bullet.position.x += bullet.velocity.speed * bullet.velocity.direction.0;
        bullet.position.y += bullet.velocity.speed * bullet.velocity.direction.1;
        println!(
            "Bullet moved to position ({}, {})",
            bullet.position.x, bullet.position.y
        );
    }
}

pub fn attack_system(towers: &Vec<Tower>, enemies: &mut [Enemy], bullets: &mut Vec<Bullet>) {
    for tower in towers {
        for enemy in enemies.iter_mut() {
            let distance = ((tower.position.x - enemy.position.x).powi(2)
                + (tower.position.y - enemy.position.y).powi(2))
            .sqrt();
            if distance <= tower.range.value {
                bullets.push(Bullet {
                    position: tower.position,
                    damage: tower.damage,
                    velocity: Velocity {
                        speed: 5.0,
                        direction: (
                            enemy.position.x - tower.position.x,
                            enemy.position.y - tower.position.y,
                        ),
                    },
                });
                println!(
                    "Tower at position ({}, {}) attacked enemy at position ({}, {})",
                    tower.position.x, tower.position.y, enemy.position.x, enemy.position.y
                );
                break;
            }
        }
    }
}

pub fn health_system(enemies: &mut Vec<Enemy>) {
    enemies.retain(|enemy| enemy.health.current > 0);
}
