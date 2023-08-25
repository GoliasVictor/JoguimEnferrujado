
use crate::world::World;

pub mod colision_system;
pub mod speed_system;

pub trait System {
	fn run(&self, a: &mut World);
}