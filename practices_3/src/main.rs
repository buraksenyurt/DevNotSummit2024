use bevy::prelude::*;

fn main() {
    let mut world = World::new();

    let mut orc_1 = world.spawn_empty();
    orc_1.insert((Position { x: 100.0, y: 0.0 }, Health(100),Speed(1.0),Power(2)));

    let mut orc_2 = world.spawn_empty();
    orc_2.insert((Position { x: 0.0, y: -100.0 }, Health(100),Speed(1.0),Power(2)));
    //println!("{}", &left_tower.id());

    let mut archer = world.spawn_empty();
    archer.insert((Position { x: 100.0, y: 0.0 }, Speed(5.0),Health(100), Power(5)));
    //println!("{}", &archer.id());

    let mut planner = Schedule::default();
    planner.add_systems((hit_damage, print_health).chain());
    planner.run(&mut world);
}

fn hit_damage(mut query: Query<(&mut Health, &Power)>) {
    for (mut h, p) in query.iter_mut() {
        h.0 -= p.0;
    }
}

fn print_health(query: Query<(&Health)>) {
    for h in query.iter() {
        println!("{:?}", h.0);
    }
}

#[derive(Debug, Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
struct Speed(f32);

#[derive(Debug, Component)]
struct Health(u32);

#[derive(Debug, Component)]
struct Power(u32);
