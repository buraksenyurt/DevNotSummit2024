use std::any::Any;
use std::cell::{RefCell, RefMut};

// Components
pub struct Health(i32);
pub struct Attack(i32);
pub struct Name(&'static str);

trait Component {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> Component for RefCell<Vec<Option<T>>> {
    fn push_none(&mut self) {
        self.get_mut().push(None)
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub struct World {
    entities_count: usize,
    components: Vec<Box<dyn Component>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            components: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let id = self.entities_count;
        for c in self.components.iter_mut() {
            c.push_none();
        }
        self.entities_count += 1;
        id
    }

    pub fn add_component<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for c in self.components.iter_mut() {
            if let Some(comp_vec) = c
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                comp_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        let mut new_components: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_components.push(None);
        }

        new_components[entity] = Some(component);
        self.components.push(Box::new(RefCell::new(new_components)));
    }

    pub fn borrow_components<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for c in self.components.iter() {
            if let Some(comp) = c
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(comp.borrow_mut());
            }
        }
        None
    }
}

fn main() {
    let mut game_world = World::new();
    let gandalf = game_world.new_entity();
    let heimdall = game_world.new_entity();
    let tower = game_world.new_entity();

    game_world.add_component(gandalf, Name("Gandalf the grey"));
    game_world.add_component(gandalf, Health(100));
    game_world.add_component(gandalf, Attack(10));

    game_world.add_component(heimdall, Name("Observer of the universe"));
    game_world.add_component(heimdall, Health(-20));
    game_world.add_component(heimdall, Attack(5));

    game_world.add_component(tower, Name("Tower Left"));
    game_world.add_component(tower, Health(1000));

    let mut healths = game_world.borrow_components::<Health>().unwrap();
    let mut names = game_world.borrow_components::<Name>().unwrap();
    let mut attacks = game_world.borrow_components::<Attack>().unwrap();

    let attackers = healths
        .iter_mut()
        .zip(names.iter_mut())
        .zip(attacks.iter_mut())
        .filter_map(|((health, name), attack)| {
            Some((health.as_mut()?, name.as_mut()?, attack.as_mut()?))
        });

    for (health, name, attack) in attackers {
        if health.0 < 0 {
            println!("{} has been dead on health {}", name.0, health.0);
        } else {
            println!(
                "{} is still healthy and has {} attack power.",
                name.0, attack.0
            );
            attack.0 += 1;
            println!("Attack power increased to {}", attack.0);
        }
    }
}
