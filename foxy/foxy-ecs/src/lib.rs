//https://ianjk.com/ecs-in-rust/
// also heavily based on my C++ framework Flugel
pub mod component;
pub mod entity;
pub mod system;

use crate::{
  component::*,
  entity::*,
  //system::*,
};
use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use lazy_static::lazy_static;
use tracing::{error, warn};
use foxy_macro::Component;

lazy_static! {
  pub static ref ECS: Mutex<ECSManager> = Mutex::new(ECSManager::new());
}

pub struct ECSManager {
  entity_components: HashMap<EntityID, HashSet<TypeId>>,
  component_maps: HashMap<TypeId, Box<dyn ComponentHashMap + Send + Sync>>,
}

impl ECSManager {
  pub fn new() -> Self {
    Self {
      entity_components: HashMap::new(),
      component_maps: HashMap::new(),
    }
  }

  pub fn spawn(&mut self) -> Entity {
    let id = uuid::Uuid::new_v4();
    self.entity_components.insert(id, HashSet::new());
    Entity {
      id
    }
  }

  /*
    TODO: Turn this into a procedural macro to register all components ahead of time.
      This will circumvent the issue of letting old entities know about new components
      without having to iterate through all the entities. Also this will move the need
      for Option<> over to the component_maps, allowing for entity_components HashMap
      to be removed. https://www.youtube.com/watch?v=crWfcA064is
  */
  fn register_component<T: 'static + Component>(&mut self) {
    self.component_maps.insert(TypeId::of::<T>(), Box::new(ComponentSparseVec::<T>::new()));
  }

  fn component_registered<T: 'static + Component>(&mut self) -> bool {
    self.component_maps.contains_key(&TypeId::of::<T>())
  }

  pub fn try_register_component<T: 'static + Component>(&mut self) -> bool {
    if !self.component_registered::<T>() {
      self.register_component::<T>();
      return true;
    }
    false
  }

  pub fn add_component<T: 'static + Component>(&mut self, entity: &Entity, component: T) {
    self.try_register_component::<T>();
    let type_id = TypeId::of::<T>();
    match self.entity_components[&entity.id].get(&type_id) {
      None => { self.entity_components.get_mut(&entity.id).unwrap().insert(type_id); }
      Some(_) => {}
    }
    match self.component_maps.get_mut(&type_id) {
      None => {
        error!("Component not registered!")
      }
      Some(component_map) => {
        component_map.as_any_mut().downcast_mut::<ComponentSparseVec<T>>().unwrap()
          .insert(entity, component);
      }
    }
  }

  pub fn remove_component<T: 'static + Component>(&mut self, entity: &Entity) {
    let type_id = TypeId::of::<T>();
    if !self.entity_components.get_mut(&entity.id).unwrap().remove(&type_id) {
      error!("Attempted removal of non-existent component!");
    }
    match self.component_maps.get_mut(&type_id) {
      None => {
        warn!("Attempted removal of non-registered component!");
      }
      Some(component_map) => {
        component_map.as_any_mut().downcast_mut::<ComponentSparseVec<T>>().unwrap()
          .remove(entity);
      }
    }
  }

  pub fn set<T: 'static + Component>(&mut self, entity: &Entity, value: T) {
    let type_id = TypeId::of::<T>();
    if self.entity_components[&entity.id].get(&type_id).is_none() {
      error!("Attempted access of non-existent component!");
    }
    match self.component_maps.get_mut(&type_id) {
      None => {
        error!("Attempted access of non-registered component!");
      }
      Some(component_map) => {
        component_map.as_any_mut().downcast_mut::<ComponentSparseVec<T>>().unwrap().set(entity, value);
      }
    }
  }

  pub fn get<T: 'static + Component>(& self, entity: &Entity) -> Option<T> {
    let type_id = TypeId::of::<T>();
    if self.entity_components[&entity.id].get(&type_id).is_none() {
      error!("Attempted access of non-existent component!");
      return None;
    }
    match self.component_maps.get(&type_id) {
      None => {
        error!("Attempted access of non-registered component!");
        None
      }
      Some(component_map) => {
        component_map.as_any().downcast_ref::<ComponentSparseVec<T>>().unwrap().get(entity)
      }
    }
  }
}

impl Default for ECSManager {
  fn default() -> Self {
    ECSManager::new()
  }
}