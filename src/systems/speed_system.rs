use crate::{World, components::{Position, Velocity}};

pub fn translate(world: &mut World, delta_time : f32){
	
	for entity in world.get_entities().iter_mut() {
		let Some(speed) = entity.get_component::<Velocity>() else { continue; };
		let Some(position) =  entity.get_component_mut::<Position>() else {  continue };
		(*position).0 = (*position).0 + speed.0 * delta_time;
	};
}

