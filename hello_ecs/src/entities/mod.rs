use crate::components::*;

#[derive(Debug)]
pub struct Tower {
    pub position: Position,
    pub health: Health,
    pub range: Range,
    pub damage: Damage,
}
#[derive(Debug)]
pub struct Enemy {
    pub position: Position,
    pub health: Health,
    pub velocity: Velocity,
}

#[derive(Debug)]
pub struct Bullet {
    pub position: Position,
    pub damage: Damage,
    pub velocity: Velocity,
}
