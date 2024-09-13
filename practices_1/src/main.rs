use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Default, Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Default, Debug)]
struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
struct Actor {
    pub name: String,
}

fn main() {
    let mut world = World::new();

    let mut warrior = world.spawn_empty();
    warrior.insert((
        Actor {
            name: "Durin".to_string(),
        },
        Position::default(),
        Velocity { x: 10, y: 10 },
    ));

    let mut legolas = world.spawn_empty();
    legolas.insert((
        Actor {
            name: "Legolas".to_string(),
        },
        Position::default(),
        Velocity { x: 20, y: 20 },
    ));

    let mut tower = world.spawn_empty();
    tower.insert(Position::default());

    let mut runner = Schedule::default();

    runner.add_systems(locate_all.after(movement));

    runner.run(&mut world);
}

fn locate_all(mut query: Query<&mut Position>) {
    let mut rnd = rand::thread_rng();
    for mut p in query.iter_mut() {
        p.x = rnd.gen_range(0..100);
        p.y = 0;
    }
}

fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut p, v) in query.iter_mut() {
        p.x += v.x;
        p.y += v.y;
    }
}
