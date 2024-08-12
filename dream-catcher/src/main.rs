use bevy::math::*;
use bevy::prelude::*;

const SPACESHIP_SIZE: Vec2 = Vec2::new(100., 15.);
const SPACESHIP_COLOR: Color = Color::srgb(0.4157, 0.6588, 0.3098);
const SPACESHIP_SPEED: f32 = 500.;

#[derive(Component)]
struct Spaceship;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.4353, 0.6588, 0.8627)))
        .add_systems(Update, bevy::window::close_when_requested)
        .add_systems(Startup, setup_system)
        .add_systems(FixedUpdate, move_spaceship)
        .run();
}

fn setup_system(mut commands: Commands, window: Query<&Window>) {
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

    let new_x = ship_transform.translation.x
        + direction * SPACESHIP_SPEED * time_step.delta().as_secs_f32();

    ship_transform.translation.x = new_x;
}
