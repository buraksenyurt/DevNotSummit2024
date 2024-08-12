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

    let mut schedule = Schedule::default();
    schedule.add_systems(show_actors);
    schedule.run(&mut world);
}

fn show_actors(query: Query<&Position>) {
    for p in &query {
        println!("Position of entity {:?}", p);
    }
}
