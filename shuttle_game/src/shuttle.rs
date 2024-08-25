use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::app::{App, Plugin, Startup};
use bevy::math::Vec3;
use bevy::prelude::*;

const INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub struct ShuttlePlugin;
impl Plugin for ShuttlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shuttle);
    }
}
fn spawn_shuttle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(MovingObjectBundle {
        velocity: Velocity { value: VELOCITY },
        acceleration: Acceleration::new(Vec3::ZERO),
        model: SceneBundle {
            scene: asset_server.load("shuttle.glb#Scene0"),
            transform: Transform::from_translation(INITIAL_POSITION),
            ..default()
        },
    });
}
