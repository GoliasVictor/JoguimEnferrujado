
use std::any::Any;
use crate::prelude::*;
mod component_box;
pub use component_box::*;

#[derive(Debug, Clone)]
pub struct Position(pub Vec2);

pub struct CompSize(pub Size);

#[derive(Debug, Clone)]
pub struct Velocity(pub Vec2);
pub trait Component : Any {

}




