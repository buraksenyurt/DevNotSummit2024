#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Velocity {
    pub speed: f32,
    pub direction: (f32, f32),
}

#[derive(Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Damage {
    pub amount: i32,
}

#[derive(Debug)]
pub struct Range {
    pub value: f32,
}
