#![feature(map_many_mut)]
#[macro_use]
pub mod bases;
pub mod components;
pub mod nodes;
pub mod systems;
pub mod world;

pub use crate::world::*;
pub use systems::*;
pub mod prelude {
    pub use crate::bases::*;
    pub use crate::components::Component;
    pub use crate::world::World;
}

#[cfg(test)]
mod tests {
    use super::*;

    use bases::Vec2;
    use components::{Position, Velocity};

    use crate::speed_system::translate;
    fn debug_world(world: &mut World) {
        let mut iter = world.get_entities();
        iter.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
        println!("entities:");
        for entity in iter {
            println!(
                " {:?}: {:?}",
                entity.id,
                (
                    entity.get_component::<Position>(),
                    entity.get_component::<Velocity>()
                )
            );
        }
    }

    #[test]
    fn geral() {
        let mut world = World::new();

        world.add_component(Position(Vec2::ZERO), 0);

        world.add_component(Position(Vec2::ZERO), 1);
        world.add_component(Velocity(Vec2::RIGHT), 1);

        world.add_component(Position(Vec2::ZERO), 2);

        world.add_component(Position(Vec2::ZERO), 3);

        translate(&mut world, 1.);
        debug_world(&mut world);
        translate(&mut world, 0.5);
        debug_world(&mut world);
    }
}
