use foxy_macro::*;
use foxy_macro_derive::*;
use std::any::Any;
use std::collections::HashMap;
use tracing::error;
use crate::{Entity, EntityID};

#[derive(Component)]
pub struct Time(pub f64);

#[derive(Component)]
pub struct Name(pub String);

pub trait ComponentHashMap {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentHashMap for ComponentSparseVec<T> {
  fn as_any(&self) -> &dyn Any {
    self as &dyn Any
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any
  }
}

pub struct ComponentSparseVec<T> {
  pub components: Vec<T>,
  pub entity_to_index: HashMap<EntityID, usize>,
  pub index_to_entity: HashMap<usize, EntityID>,
}

impl<T> ComponentSparseVec<T> {
  pub fn new() -> Self {
    Self {
      components: Vec::new(),
      entity_to_index: HashMap::new(),
      index_to_entity: HashMap::new(),
    }
  }

  pub fn insert(&mut self, entity: &Entity, value: T) {
    match self.entity_to_index.get(&entity.id) {
      Some(_) => { error!("Attempted component access upon non-existent data!"); }
      None => {
        *self.entity_to_index.get_mut(&entity.id).unwrap() = self.components.len();
        *self.index_to_entity.get_mut(&self.components.len()).unwrap() = entity.id;
        self.components.push(value);
      }
    }
  }

  pub fn remove(&mut self, entity: &Entity) {
    // get ids
    let index_last = self.components.len() - 1;

    // Surrounded in block to drop borrow
    let (index_target, entity_last) = {
      let index_target = self.entity_to_index.get(&entity.id);
      if index_target.is_none() {
        error!("Attempted component removal upon non-existent data!");
      }
      let entity_last = self.index_to_entity.get(&index_last);

      (Some(*index_target.unwrap()), *entity_last.unwrap())
    };

    match index_target {
      None => {}
      Some(index_target) => {
        // move target to end of vector and pop end
        self.components.swap_remove(index_target);

        // cleanup
        *self.entity_to_index.get_mut(&entity_last).unwrap() = index_target;
        *self.index_to_entity.get_mut(&index_target).unwrap() = entity_last;
        self.entity_to_index.remove(&entity.id);
        self.index_to_entity.remove(&index_last);
      }
    }
  }

  pub fn set(&mut self, entity: &Entity, value: T) {
    match self.entity_to_index.get(&entity.id) {
      Some(index) => self.components[*index] = value,
      None => error!("Attempted component access upon non-existent data!"),
    }
  }

  pub fn get(&self, entity: &Entity) -> Option<T> {
    self.entity_to_index.get(&entity.id).map(|index| self.components.get(*index))
  }
}

impl<T: Clone> Default for ComponentSparseVec<T> {
  fn default() -> Self {
    ComponentSparseVec::new()
  }
}