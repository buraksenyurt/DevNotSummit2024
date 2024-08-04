// Components
pub struct Health(i32);
pub struct Attack(i32);
pub struct Id(&'static str);

pub struct World {
    pub healths: Vec<Option<Health>>,
    pub attacks: Vec<Option<Attack>>,
    pub ids: Vec<Option<Id>>,
}
impl World {
    pub fn new() -> Self {
        Self {
            healths: Vec::new(),
            attacks: Vec::new(),
            ids: Vec::new(),
        }
    }

    pub fn create_entity(
        &mut self,
        health: Option<Health>,
        attack: Option<Attack>,
        id: Option<Id>,
    ) {
        self.healths.push(health);
        self.attacks.push(attack);
        self.ids.push(id);
    }
}
fn main() {
    let mut game_world = World::new();
    game_world.create_entity(Some(Health(40)), Some(Attack(5)), Some(Id("Azurat")));
    game_world.create_entity(Some(Health(80)), Some(Attack(2)), Some(Id("Nazbendiyr")));
    game_world.create_entity(Some(Health(1000)), None, Some(Id("CrakenGate")))
}
