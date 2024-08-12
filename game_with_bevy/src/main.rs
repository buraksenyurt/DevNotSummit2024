use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Goal;

#[derive(Component)]
struct Speed(f32);
#[derive(Component)]
struct Position(f32);
#[derive(Component)]
struct Health(i32);

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, enemy_movement, check_goal))
        .run();
}

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    commands.spawn((
        Player,
        Speed(1.0),
        Health(100),
        Position(rng.gen_range(5.0..10.0)),
    ));
    commands.spawn((
        Enemy,
        Speed(1.10),
        Health(100),
        Position(rng.gen_range(5.0..10.0)),
    ));
    commands.spawn((Goal, Speed(1.0), Position(20.)));
}

fn player_movement(mut query: Query<(&Speed, &mut Position), With<Player>>) {
    for (speed, mut position) in query.iter_mut() {
        position.0 += speed.0;
        println!("player is moving to {}", position.0);
    }
}

fn enemy_movement(mut query: Query<(&Speed, &mut Position), With<Enemy>>) {
    let mut rng = rand::thread_rng();
    for (speed, mut position) in query.iter_mut() {
        let _direction: f32 = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        position.0 += speed.0;
        println!("enemy is moving to {}", position.0);
    }
}

fn check_goal(
    player_query: Query<&Position, With<Player>>,
    goal_query: Query<&Position, With<Goal>>,
) {
    for player_position in player_query.iter() {
        for goal_position in goal_query.iter() {
            if (player_position.0 - goal_position.0).abs() < 0.5 {
                println!("You reached the goal!");
            }
        }
    }
}
