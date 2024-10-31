// Klasik Senaryo

struct GameActor {
    id: String,
    health: f32,
}

trait Damage {
    fn damage(&mut self, amount: f32);
}

impl GameActor {
    fn new(id: &str, health_value: f32) -> Self {
        Self {
            id: id.to_string(),
            health: health_value,
        }
    }
}

impl Damage for GameActor {
    fn damage(&mut self, amount: f32) {
        self.health -= amount;
    }
}

struct Archer {
    base: GameActor,
    range: f32,
}

impl Archer {
    fn new(id: &str, health_value: f32, range: f32) -> Self {
        Self {
            base: GameActor::new(id, health_value),
            range,
        }
    }
    fn fire(&self) {
        println!(
            "Archer {} fires at well with range {} unit",
            self.base.id, self.range
        );
    }
}

struct Warrior {
    base: GameActor,
    strength: f32,
}

impl Warrior {
    fn new(id: &str, health_value: f32, strength: f32) -> Self {
        Self {
            base: GameActor::new(id, health_value),
            strength,
        }
    }
    fn attack(&self) {
        println!(
            "Warrior {} attacks with strength {} power",
            self.base.id, self.strength
        );
    }
}

struct Healer {
    base: GameActor,
    mana_power: f32,
}

impl Healer {
    fn new(name: &str, health_value: f32, mana_power: f32) -> Self {
        Self {
            base: GameActor::new(name, health_value),
            mana_power,
        }
    }

    fn heal(&self) {
        println!("{} heals with power {} mana", self.base.id, self.mana_power);
    }
}

pub fn run() {
    let white_hand = Archer::new("wh-666", 100.0, 900.50);
    let mut boramir = Warrior::new("boramir-13", 100.0, 8.75);
    let gandalf = Healer::new("gandalf-13", 100.0, 1.25);

    white_hand.fire();
    boramir.attack();
    boramir.base.damage(10.5);
    gandalf.heal();
    println!("Boramir health {}", boramir.base.health);
}
