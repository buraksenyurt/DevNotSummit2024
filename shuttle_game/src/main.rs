mod camera;
mod log;
mod movement;
mod shuttle;

use crate::camera::CameraPlugin;
use crate::log::LogPlugin;
use crate::movement::MovementPlugin;
use crate::shuttle::ShuttlePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.25)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.,
        })
        // User plugins
        .add_plugins(ShuttlePlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(LogPlugin)
        .run();
}
