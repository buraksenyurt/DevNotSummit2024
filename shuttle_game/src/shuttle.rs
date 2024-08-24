use crate::movement::Velocity;
use bevy::app::{App, Plugin, Startup};
use bevy::math::Vec3;
use bevy::prelude::*;

const INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
struct ShuttleBundle {
    velocity: Velocity,
    model: SceneBundle,
}

pub struct ShuttlePlugin;
impl Plugin for ShuttlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shuttle);
    }
}
fn spawn_shuttle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ShuttleBundle {
        velocity: Velocity { value: VELOCITY },
        model: SceneBundle {
            scene: asset_server.load("shuttle.glb#Scene0"),
            transform: Transform::from_translation(INITIAL_POSITION),
            ..default()
        },
    });
}
