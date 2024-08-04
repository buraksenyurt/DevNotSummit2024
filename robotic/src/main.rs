// Components
struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    dx: f32,
    dy: f32,
}

struct Sensor {
    sensor_type: String,
    value: f32,
}

struct Arm {
    length: f32,
    angle: f32,
}

// Entity
struct Robot {
    position: Position,
    velocity: Velocity,
    sensors: Vec<Sensor>,
    arms: Vec<Arm>,
}

// Systems
fn update_position(robots: &mut Vec<Robot>, dt: f32) {
    for robot in robots.iter_mut() {
        robot.position.x += robot.velocity.dx * dt;
        robot.position.y += robot.velocity.dy * dt;
    }
}

fn read_sensors(robots: &mut Vec<Robot>) {
    for robot in robots.iter_mut() {
        for sensor in robot.sensors.iter_mut() {
            sensor.value = read_sensor(&sensor.sensor_type);
        }
    }
}

fn control_arms(robots: &mut Vec<Robot>, target_positions: Vec<(f32, f32)>) {
    for (robot, target) in robots.iter_mut().zip(target_positions.iter()) {
        for arm in robot.arms.iter_mut() {
            let target_angle = calc_angle(&robot.position, &target);
            arm.angle = (arm.angle + target_angle) / 2.0;
        }
    }
}

fn calc_angle(position: &Position, new_position: &(f32, f32)) -> f32 {
    println!(
        "Moving to new position {}:{}",
        new_position.0, new_position.1
    );
    1.50
}

fn read_sensor(sensor_type: &String) -> f32 {
    println!("Reading sensor values for {sensor_type}");
    3.50
}

fn main() {
    let mut robots = vec![
        Robot {
            position: Position { x: 0.0, y: 0.0 },
            velocity: Velocity { dx: 1.0, dy: 0.5 },
            sensors: vec![Sensor {
                sensor_type: String::from("Temperature"),
                value: 0.0,
            }],
            arms: vec![Arm {
                length: 1.0,
                angle: 0.0,
            }],
        },
        Robot {
            position: Position { x: 10.0, y: 10.0 },
            velocity: Velocity { dx: 1.0, dy: 0.25 },
            sensors: vec![Sensor {
                sensor_type: String::from("Range"),
                value: 0.0,
            }],
            arms: vec![Arm {
                length: 1.0,
                angle: 0.0,
            }],
        },
    ];

    let dt = 0.5;
    update_position(&mut robots, dt);
    read_sensors(&mut robots);
    control_arms(&mut robots, vec![(32.0, 45.0)]);
}
