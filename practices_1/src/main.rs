use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
struct Actor {
    name: String,
}

fn main() {
    let world = World::new();
}
