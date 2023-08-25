use std::{ops, fmt, fmt::{Formatter,Result}};


macro_rules! vec2 {
	($x:expr,$y:expr)=>{
		 Vec2 { x: $x, y: $y}
	}
}


#[derive(Clone, Copy)]
pub struct Vec2 { 
	pub x : f32,
	pub y : f32,

}


impl fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({x}, {y})", x=self.x, y=self.y)
    }
}


impl Vec2 {
	
	pub fn new(x : f32, y:f32) -> Vec2{
		Vec2{x, y} 
	}
	pub const LEFT: Vec2 = vec2!(-1., 0.);
	pub const RIGHT: Vec2 = vec2!(1., 0.);
	pub const UP: Vec2 = vec2!(0., 1.);
	pub const DOWN: Vec2 = vec2!(0., -1.);
	pub const ZERO: Vec2 = vec2!(0., 0.);
}
impl ops::Add<Vec2> for Vec2 {
	type Output = Vec2;
	fn add(self, rhs: Vec2) -> Self::Output {
		return Vec2::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl ops::Sub<Vec2> for Vec2 {
	type Output = Vec2;
	fn sub(self, rhs: Vec2) -> Self::Output {
		return Vec2::new(self.x - rhs.x, self.y - rhs.y)
	}
}

impl ops::Mul<f32> for Vec2 {
	type Output = Vec2;
	fn mul(self, rhs: f32) -> Self::Output {
		return Vec2::new(self.x  * rhs, self.y * rhs)
	}
}
impl ops::Div<f32> for Vec2 {
	type Output = Vec2;
	fn div(self, rhs: f32) -> Self::Output {
		return Vec2::new(self.x  / rhs, self.y / rhs)
	}
}

