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
    game_world.create_entity(Some(Health(-5)), Some(Attack(5)), Some(Id("Mithrandir")));
    game_world.create_entity(Some(Health(80)), Some(Attack(2)), Some(Id("Nazbendiyr")));
    game_world.create_entity(Some(Health(1000)), None, Some(Id("Tower_1")));
    game_world.create_entity(Some(Health(50)), None, Some(Id("Artillary_1")));
    game_world.create_entity(
        Some(Health(-1)),
        Some(Attack(1)),
        Some(Id("Villager_Jonas")),
    );

    let attackers = game_world
        .healths
        .iter()
        .zip(game_world.attacks.iter())
        .zip(game_world.ids.iter());

    let attackers_with_ids = attackers.filter_map(
        |((health, attack), id): ((&Option<Health>, &Option<Attack>), &Option<Id>)| {
            Some((health.as_ref()?, attack.as_ref()?, id.as_ref()?))
        },
    );

    for (h, a, n) in attackers_with_ids {
        if h.0 < 0 {
            println!("{} has been dead!", n.0);
        } else {
            println!("{} is still healthy and has {} attack power", n.0, a.0);
        }
    }
}
