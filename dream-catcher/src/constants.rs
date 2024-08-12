use bevy::color::Color;
use bevy::math::{Vec2, Vec3};

pub const SPACESHIP_SIZE: Vec2 = Vec2::new(100., 15.);
pub const SPACESHIP_COLOR: Color = Color::srgb(0.4157, 0.6588, 0.3098);
pub const SPACESHIP_SPEED: f32 = 600.;
pub const BALL_STARTING_POS: Vec3 = Vec3::new(0., -250., 1.);
pub const BALL_SIZE: Vec2 = Vec2::new(32., 32.);
pub const BALL_SPEED: f32 = 500.;
pub const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);