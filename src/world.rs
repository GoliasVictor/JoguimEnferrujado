use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    vec,
};

use crate::{
    components::ComponentBox,
    nodes::{EntitieData, EntitieId},
};

pub struct World {
    all_components: HashMap<TypeId, HashMap<EntitieId, ComponentBox>>,
    all_entities: HashSet<EntitieId>,
}

impl World {
    pub fn new() -> Self {
        World {
            all_components: HashMap::new(),
            all_entities: HashSet::new(),
        }
    }
    pub fn add_component<T: Any>(&mut self, component: T, entitie_id: EntitieId) {
        let tid = TypeId::of::<T>();

        if  !self.all_entities.contains(&entitie_id){
            self.all_entities.insert(entitie_id);
        }
        if !self.all_components.contains_key(&tid) {
            self.all_components.insert(tid, HashMap::from([]));
        }
        if let Some(components) = self.all_components.get_mut(&TypeId::of::<T>()) {
            components.insert(
                entitie_id,
                ComponentBox::new(Box::new(component), entitie_id),
            );
        }
    }
    pub fn get_components<T: Any>(&self) -> Option<Vec<&T>> {
        if let Some(components) = self.all_components.get(&TypeId::of::<T>()) {
            return Some(
                components
                    .iter()
                    .filter_map(|(_, comp_box)| comp_box.component.downcast_ref::<T>())
                    .collect(),
            );
        }
        return None;
    }

    pub fn get_entities<'a>(&'a mut self) -> Vec<EntitieData<'a>> {
        let mut entities: HashMap<EntitieId, HashMap<TypeId, &mut Box<dyn Any>>> = HashMap::new();
        
        for (tid, comps) in self.all_components.iter_mut() {
            
            // safe because each entitie has only one component of one type, never is created a mut ref of the same value 
            let comps_ptr = comps as *mut HashMap<EntitieId, ComponentBox>;
            
            for id in self.all_entities.iter() {
                
                let comps = unsafe { &mut *comps_ptr };

                if let Some(comp) = comps.get_mut(id) {
                    entities
                        .entry(*id)
                        .or_insert_with(HashMap::new)
                        .insert(*tid, &mut comp.component);
                }
            }
        }
        return entities
            .into_iter()
            .map(|(id, components)| EntitieData::new(id, components))
            .collect();
    }
    pub fn get_entities_mut_with<T: Any>(&mut self) -> Vec<&mut EntitieData> {
        return vec![];
    }
    pub fn get_components_mut<T: Any>(&mut self) -> Option<Vec<&mut T>> {
        self.all_components
            .get_mut(&TypeId::of::<T>())
            .map(|components| {
                components
                    .iter_mut()
                    .filter_map(|(_, comp_box)| comp_box.component.downcast_mut::<T>())
                    .collect()
            })
    }
}
