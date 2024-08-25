mod camera;
mod log;
mod movement;
mod pickup_crate;
mod shuttle;

use crate::camera::CameraPlugin;
use crate::movement::MovementPlugin;
use crate::pickup_crate::PickupPlugin;
use crate::shuttle::ShuttlePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.50)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.,
        })
        // User plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(MovementPlugin)
        .add_plugins(ShuttlePlugin)
        .add_plugins(PickupPlugin)
        //.add_plugins(LogPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
