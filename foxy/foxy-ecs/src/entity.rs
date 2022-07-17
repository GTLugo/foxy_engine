pub use uuid::Uuid as EntityID;
use foxy_macro::Component;
use crate::ECS;

pub struct Entity {
  pub id: EntityID,
}

impl Entity {
  pub fn new() -> Self {
    ECS.lock().unwrap().spawn()
  }

  pub fn add_component<T: 'static + Component>(&mut self, component: T) -> &Self {
    ECS.lock().unwrap().add_component(self, component);
    self
  }

  pub fn remove_component<T: 'static + Component>(&mut self) -> &Self {
    ECS.lock().unwrap().remove_component::<T>(self);
    self
  }

  pub fn set<T: 'static + Component>(&mut self, value: T) {
    ECS.lock().unwrap().set::<T>(self, value);
  }

  pub fn get<T: 'static + Component>(&self) -> Option<T> {
    ECS.lock().unwrap().get::<T>(self)
  }
}

impl Default for Entity {
  fn default() -> Self {
    Entity::new()
  }
}