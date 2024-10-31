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

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Resource)]
struct Timer(f32);

fn setup_level_alpha(query: Query<(Entity, &Position)>) {
    println!("Setup system");
    for (entity, position) in query.iter() {
        println!("{:?}\t{:?}. ", entity, position);
    }
}

fn log_players(query: Query<&Position, With<Player>>) {
    println!("Log player positions");
    for position in query.iter() {
        println!("Player on {:?}. ", position);
    }
}

fn log_enemies(query: Query<(&Position, &Velocity), Without<Player>>) {
    println!("Log enemy positions");
    for (position, _) in query.iter() {
        println!("Enemy go to position {:?}. ", position);
    }
}

fn move_all_enemies(mut query: Query<(&mut Position, &Velocity), Without<Player>>, timer: Res<Timer>) {
    println!("Moving Enemies");
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x * timer.0;
        position.y += velocity.y * timer.0;
    }
}

pub fn run() {
    let mut world = World::new();

    let mut aragon = world.spawn_empty();
    aragon.insert((
        Position { x: 10.0, y: 0.0 },
        Velocity { x: 1.0, y: 0.0 },
        Player,
    ));

    let mut legolas = world.spawn_empty();
    legolas.insert((
        Position { x: 16.0, y: 0.0 },
        Velocity { x: 1.0, y: 0.0 },
        Player,
    ));

    let mut orc_warrior = world.spawn_empty();
    orc_warrior.insert((Position { x: 50.0, y: 0.0 }, Velocity { x: -1.0, y: 0.0 }));

    let mut tower = world.spawn_empty();
    tower.insert(Position { x: 25.0, y: 25.0 });

    world.insert_resource(Timer(0.2));

    let mut schedule = Schedule::default();
    schedule.add_systems((
        (setup_level_alpha, log_players, log_enemies).chain(),
        move_all_enemies
            .after(setup_level_alpha)
            .before(log_players)
            .before(log_enemies),
    ));

    schedule.run(&mut world);
}
