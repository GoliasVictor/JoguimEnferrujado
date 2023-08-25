use crate::nodes::EntitieId;

use std::any::Any;

pub struct ComponentBox { 
	    pub component: Box<dyn Any>,
	    pub entitie_id : usize
}

impl ComponentBox {
	    pub fn new( component: Box<dyn Any>, entitie_id : EntitieId) -> Self {
		    return Self { component, entitie_id }
	    }
}
