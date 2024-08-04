use std::any::Any;

// Components
pub struct Health(i32);
pub struct Attack(i32);

trait Component {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> Component for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None)
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

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for c in self.components.iter_mut() {
            if let Some(comp_vec) = c.as_any_mut().downcast_mut::<Vec<Option<ComponentType>>>() {
                comp_vec[entity] = Some(component);
                return;
            }
        }

        let mut new_components: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_components.push(None);
        }

        new_components[entity] = Some(component);
        self.components.push(Box::new(new_components));
    }
}
fn main() {
    let mut game_world = World::new();
    let gandalf = game_world.new_entity();
    let heimdall = game_world.new_entity();

    game_world.add_component_to_entity(gandalf, Health(100));
    game_world.add_component_to_entity(gandalf, Attack(10));

    game_world.add_component_to_entity(heimdall, Health(80));
    game_world.add_component_to_entity(heimdall, Attack(5));

    println!("Entities and components has been added.");
}
