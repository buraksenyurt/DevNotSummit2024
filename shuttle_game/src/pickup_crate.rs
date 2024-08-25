use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = -25.0..25.0;
const VELOCITY_SCALE: f32 = 2.5;
const ACCELERATION_SCALE: f32 = 1.0;
const SPAWN_TIME_SECONDS: f32 = 5.0;
const SCALE_FACTOR: f32 = 3.0;

#[derive(Component, Debug)]
pub struct PickupCrate;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_meteor);
    }
}

fn spawn_meteor(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );
    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALE;
    let acceleration = random_unit_vector() * ACCELERATION_SCALE;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            model: SceneBundle {
                scene: asset_server.load("crate.glb#Scene0"),
                transform: Transform {
                    translation,
                    scale: Vec3::splat(SCALE_FACTOR),
                    ..default()
                },
                ..default()
            },
        },
        PickupCrate,
    ));
}
