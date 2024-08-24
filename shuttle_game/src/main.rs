use bevy::prelude::*;

#[derive(Component, Debug)]
struct Velocity {
    pub value: Vec3,
}

pub struct ShuttlePlugin;

impl Plugin for ShuttlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shuttle);
    }
}

fn spawn_shuttle(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), Velocity { value: Vec3::ZERO }));
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

pub struct InfoPlugin;

impl Plugin for InfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, log_position);
    }
}

fn update_position(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in query.iter_mut() {
        transform.translation.x += vel.value.x;
        transform.translation.y += vel.value.y;
        transform.translation.z += vel.value.z;
    }
}

fn log_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} at {:?}", entity, transform.translation);
    }
}

fn main() {
    App::new()
        .add_plugins(ShuttlePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(InfoPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
