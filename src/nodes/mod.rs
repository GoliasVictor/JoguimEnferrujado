use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub type EntitieId = usize;

pub trait SafeComponent: Any + Clone {}
impl<T: Any + Clone> SafeComponent for T {}
#[derive(Debug)]
pub struct EntitieData<'a> {
    pub id: EntitieId,
    components: HashMap<TypeId,&'a mut Box<dyn Any>>,
}
impl<'a> EntitieData<'a> {

    pub fn new( id: EntitieId, components: HashMap<TypeId, &'a mut Box<dyn Any>>) -> Self{
        return Self {
            id,
            components
        }
    }

    pub fn get_component<T: SafeComponent>(&self) -> Option<T> {
        let tid = TypeId::of::<T>();
        return self
            .components
            .get(&tid)
            .and_then(|cbox| cbox.downcast_ref::<T>())
            .map(Clone::clone);
    }

    pub fn get_component_ref<T: Any>(&self) -> Option<&T> {
        let tid = TypeId::of::<T>();
        return self
            .components
            .get(&tid)
            .and_then(|cbox| cbox.downcast_ref::<T>());
    }

    pub fn get_component_mut<'b, T: Any>(&'b mut self) -> Option<&'b mut T> {
        let tid = TypeId::of::<T>();
        return self
            .components
            .get_mut(&tid)
            .and_then(|cbox| cbox.downcast_mut::<T>());
    }
}
