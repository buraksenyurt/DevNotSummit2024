struct Character {
    id: String,
    strength: f32,
}

impl Character {
    fn new(id: &str, strength: f32) -> Self {
        Self {
            id: id.to_string(),
            strength,
        }
    }
    fn take_damage(&mut self, amount: f32) {
        self.strength -= amount;
    }
}

fn main() {
    println!("Hello, world!");
}
