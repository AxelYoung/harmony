use std::{cell::{RefCell, RefMut, Ref}};

mod macros;

pub struct World {
   entities_count: usize,
   component_vecs: Vec<Box<dyn ComponentVec>>
}

impl World {
   pub fn new() -> Self {
      Self {
        entities_count: 0,
        component_vecs: Vec::new(),
      }
   }

   pub fn new_entity(&mut self) -> usize {
      let entity_id = self.entities_count;
      for component_vec in self.component_vecs.iter_mut() {
        component_vec.push_none();
      }
      self.entities_count += 1;
      entity_id
   }

   pub fn add_component_to_entity <ComponentType: 'static> (
      &mut self, 
      entity: usize, 
      component: ComponentType) {
        for component_vec in self.component_vecs.iter_mut() {
           if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() {
              component_vec.get_mut()[entity] = Some(component);
              return;
           }
        }

        let mut new_component_vec : Vec<Option<ComponentType>> =
        Vec::with_capacity(self.entities_count);
      
        for _ in 0..self.entities_count {
           new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);
        self.component_vecs.push(Box::new(RefCell::new(new_component_vec)));
   }

   pub fn remove_component_from_entity <ComponentType: 'static> (
      &mut self,
      entity: usize) {
        for component_vec in self.component_vecs.iter_mut() {
           if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() {
              component_vec.get_mut()[entity] = None;
              return;
           }
        }

        let mut new_component_vec : Vec<Option<ComponentType>> =
        Vec::with_capacity(self.entities_count);
      
        for _ in 0..self.entities_count {
           new_component_vec.push(None);
        }

        self.component_vecs.push(Box::new(RefCell::new(new_component_vec)));
      }

   pub fn get_component_from_entity_mut <ComponentType: 'static> (&mut self, entity: usize) -> Option<&mut Option<ComponentType>> {
      for component_vec in self.component_vecs.iter_mut() {
        if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() {
           return Some(&mut component_vec.get_mut()[entity]);
        }
      }
      None
   }

   pub fn delete_entity (&mut self, entity: usize) {
      for component_vec in self.component_vecs.iter_mut() {
        component_vec.set_none(entity);
      }
   }

   pub fn borrow_components <ComponentType: 'static> (&self) -> Option<Ref<Vec<Option<ComponentType>>>> {
      for component_vec in self.component_vecs.iter() {
        if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
           return Some(component_vec.borrow());
        }
      }
      None
   }

   pub fn borrow_components_mut <ComponentType: 'static> (&self) -> Option<RefMut<Vec<Option<ComponentType>>>> {
      for component_vec in self.component_vecs.iter() {
        if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
           return Some(component_vec.borrow_mut());
        }
      }
      None
   }
}

trait ComponentVec {
   fn as_any(&self) -> &dyn std::any::Any;
   fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
   fn push_none(&mut self);
   fn set_none(&mut self, entity: usize);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
   fn push_none(&mut self) {
      self.get_mut().push(None);
   }

   fn set_none(&mut self, entity: usize) {
      self.get_mut()[entity] = None;
   }

   fn as_any(&self) -> &dyn std::any::Any {
      self as &dyn std::any::Any
   }

   fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
      self as &mut dyn std::any::Any
   }
}