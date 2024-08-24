use bevy::prelude::*;

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

fn spawn_shuttle(mut commands: Commands) {
    commands.spawn((Position { x: 0., y: 0. }, Velocity { x: 1., y: 1. }));
}

fn update_position(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in query.iter_mut() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

fn log_position(query: Query<(Entity, &Position)>) {
    for (entity, pos) in query.iter() {
        info!("Entity {:?} at {:?}", entity, pos);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, spawn_shuttle)
        .add_systems(Update, (update_position, log_position))
        .add_plugins(DefaultPlugins)
        .run();
}
