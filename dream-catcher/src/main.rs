mod constants;

use crate::constants::*;
use bevy::math::*;
use bevy::prelude::*;

#[derive(Component)]
struct Spaceship;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(1., 1., 1.)))
        .add_systems(Update, bevy::window::close_when_requested)
        .add_systems(Startup, setup_system)
        .add_systems(FixedUpdate, (move_spaceship, change_velocity))
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());
    let window = window.single();
    info!("Window size {:?}", window.resolution);

    // Spaceship
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(
                    0.,
                    -1.0 * window.resolution.height() * 0.5 + SPACESHIP_SIZE.y * 1.5,
                    0.,
                ),
                ..default()
            },
            sprite: Sprite {
                color: SPACESHIP_COLOR,
                custom_size: Some(SPACESHIP_SIZE),
                ..default()
            },
            ..default()
        },
        Spaceship,
    ));

    // Ball
    let ball_texture = asset_server.load("textures/ball.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POS,
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            texture: ball_texture,
            ..default()
        },
        Ball,
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
    ));
}

fn move_spaceship(
    input: Res<ButtonInput<KeyCode>>,
    time_step: Res<Time<Fixed>>,
    mut query: Query<&mut Transform, With<Spaceship>>,
) {
    let mut ship_transform = query.single_mut();
    let mut direction = 0.0;

    if input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let new_x =
        ship_transform.translation.x + direction * SPACESHIP_SPEED * time_step.delta_seconds();

    ship_transform.translation.x = new_x;
}

fn change_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time<Fixed>>) {
    let delta_time = time_step.delta_seconds();
    for (mut t, v) in &mut query {
        t.translation.x += v.x * delta_time;
        t.translation.y += v.x * delta_time;
    }
}
