use bevy::math::*;
use bevy::prelude::*;

const SPACESHIP_START_Y: f32 = 0.;
const SPACESHIP_WIDTH: Vec2 = Vec2::new(80., 20.);
const SPACESHIP_COLOR: Color = Color::srgb(1., 1., 1.);
const SPACESHIP_SPEED: f32 = 1000.;

#[derive(Component)]
struct Spaceship;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.5)))
        .add_systems(Update, bevy::window::close_when_requested)
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Spaceship
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0., SPACESHIP_START_Y, 0.),
                ..default()
            },
            sprite: Sprite {
                color: SPACESHIP_COLOR,
                custom_size: Some(SPACESHIP_WIDTH),
                ..default()
            },
            ..default()
        },
        Spaceship,
    ));
}
