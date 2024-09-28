use bevy::prelude::*;
use std::thread::sleep;
use std::time::Duration;

#[derive(Resource)]
struct Timer(f32);

fn main() {
    let mut world = World::default();
    world.insert_resource(Timer(0.2));

    let legolas = world.spawn((
        Identity {
            name: "Legolas".to_string(),
        },
        Position { x: 0.0, y: 10.0 },
        Velocity { x: 1.0, y: 0.0 },
        Archer,
    ));

    let gold_tower = world.spawn((Position { x: 10.0, y: 10.0 }));

    let mut planner = Schedule::default();
    // planner.add_systems(
    //     (
    //         check_positions,
    //         move_soldiers,
    //         move_soldiers,
    //         check_positions,
    //     )
    //         .chain(),
    // );
    // planner.run(&mut world);
    planner.add_systems((move_soldiers, check_positions).chain());
    loop {
        sleep(Duration::from_secs_f32(1.0));
        planner.run(&mut world);
    }
}

fn check_positions(query: Query<&Position>) {
    println!("Positions checking...");
    for p in query.iter() {
        println!("{:?}", p);
    }
}

fn move_soldiers(mut query: Query<(&mut Position, &Velocity), With<Archer>>, res: Res<Timer>) {
    println!("Moving Soldiers...");
    for (mut p, v) in query.iter_mut() {
        p.x += v.x * res.0;
        p.y += v.y * res.0;
    }
}

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Identity {
    name: String,
}

#[derive(Component)]
struct Archer;
