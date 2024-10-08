use bevy::prelude::*;

fn main() {
    let mut world = World::new();
    let mut legolas = world.spawn_empty();
    legolas.insert((
        Position { x: 10.0, y: 10.0 },
        Acceleration(1.5),
        Power(5),
        Player {},
    ));

    let mut orc_1 = world.spawn_empty();
    orc_1.insert((Position { x: 90.0, y: 80.0 }, Health(100)));

    let mut orc_2 = world.spawn_empty();
    orc_2.insert((Position { x: 95.0, y: 80.0 }, Health(100)));

    let mut black_tower = world.spawn_empty();
    black_tower.insert((Position { x: 45.0, y: -50.0 }, Health(1000)));

    let mut planner = Schedule::default();
    planner.add_systems((log_all, move_player, move_player, log_player).chain());
    planner.run(&mut world);
}

fn log_all(query: Query<(Entity, &Position)>) {
    for (e, pos) in query.iter() {
        println!("{:?} {:?}", e, pos);
    }
}

fn log_player(query: Query<(Entity, &Position), With<Player>>) {
    let data = query.single();
    println!("Player {:?} {:?}", data.0, data.1);
}

fn move_player(mut query: Query<(&mut Position, &Acceleration), With<Player>>) {
    let mut player = query.single_mut();
    player.0.x += player.1 .0;
}

#[derive(Debug, Component)]
struct Player {}

#[derive(Debug, Component)]

struct Health(u32);
#[derive(Debug, Component)]
struct Power(u32);

#[derive(Debug, Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
struct Direction {
    x: f32,
    y: f32,
}

#[derive(Debug, Component, Default)]
struct Acceleration(f32);
