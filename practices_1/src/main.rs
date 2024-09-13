use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Default, Debug)]
struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default, Debug)]
struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
struct Name(&'static str);

#[derive(Resource)]
struct Timer(f32);

#[derive(Resource)]
struct RunOnce(bool);

fn main() {
    let mut world = World::new();
    world.insert_resource(Timer(0.2));
    world.insert_resource(RunOnce(false));

    let mut warrior = world.spawn_empty();
    warrior.insert((
        Name("Durin"),
        Position::default(),
        Velocity { x: 1.0, y: 1.0 },
    ));

    let mut legolas = world.spawn_empty();
    legolas.insert((
        Name("Legolas"),
        Position::default(),
        Velocity { x: 1.5, y: 1.5 },
    ));

    let mut tower = world.spawn_empty();
    tower.insert(Position::default());

    let mut runner = Schedule::default();

    runner.add_systems(
        (
            setup.run_if(get_one_time_run_state),
            go,
            show_current_positions,
        )
            .chain(),
    );

    runner.run(&mut world);
    runner.run(&mut world);
    runner.run(&mut world);
}

fn setup(mut query: Query<&mut Position>, mut res: ResMut<RunOnce>) {
    println!("Locating...");
    let mut rnd = rand::thread_rng();
    for mut p in query.iter_mut() {
        p.x = rnd.gen_range(0.0..10.0);
        p.y = 0.0;
    }
    res.0 = true;
}

fn go(mut query: Query<(&mut Position, &Velocity)>, res: Res<Timer>) {
    println!("Going...");
    for (mut p, v) in query.iter_mut() {
        p.x += v.x * res.0;
        p.y += v.y * res.0;
    }
}

fn show_current_positions(_commands: Commands, query: Query<(&Name, &Position)>) {
    for (n, p) in query.iter() {
        println!("{:?} ({:?})", n.0, p);
    }
}

fn get_one_time_run_state(res: Res<RunOnce>) -> bool {
    res.0
}
