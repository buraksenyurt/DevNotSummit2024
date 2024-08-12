use bevy::prelude::*;

#[derive(Debug, Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    let mut world = World::new();

    let mut player = world.spawn_empty();
    player.insert((Position { x: 10.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));

    let mut enemy = world.spawn_empty();
    enemy.insert((Position { x: 50.0, y: 0.0 }, Velocity { x: -1.0, y: 1.0 }));

    let mut tower = world.spawn_empty();
    tower.insert(Position { x: 25.0, y: 25.0 });

    let mut schedule = Schedule::default();
    schedule.add_systems(visit_positions);
    schedule.run(&mut world);

    println!();

    schedule.add_systems(move_actors);
    schedule.run(&mut world);
}

fn visit_positions(query: Query<(Entity, &Position)>) {
    for (entity, position) in query.iter() {
        println!("{:?}\t{:?}. ", entity, position);
    }
}

fn move_actors(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}
