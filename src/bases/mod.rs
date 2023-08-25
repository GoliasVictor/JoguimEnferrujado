#[macro_use]
mod vec2;

pub use vec2::*;

   
#[derive(Clone, Copy, PartialEq)]
pub struct Size { 
	pub height : f32,
	pub width : f32
}
pub struct SidesPositions {
	pub left : f32,
	pub right : f32,
	pub top : f32,
	pub bottom : f32
}

impl SidesPositions {
	pub fn new(left:f32, right:f32, top:f32, bottom:f32) -> SidesPositions {
		SidesPositions { left, right, top, bottom }
	}
	pub fn sides_pos_size(position : Vec2, size : Size) -> SidesPositions {
		SidesPositions { 
			left:  position.x - size.width / 2.,
			right:  position.x + size.width / 2.,
			top:  position.y - size.height / 2.,
			bottom:  position.y + size.height / 2.
		}
	}
}