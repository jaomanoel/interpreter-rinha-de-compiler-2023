use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::enums::return_types::ReturnTypes;

#[derive(Debug, Default, PartialEq)]
pub struct Scope {
    pub parent: Option<Rc<Scope>>,
    pub current: Rc<RefCell<HashMap<String, ReturnTypes>>>,
}

impl Scope {
    pub fn get(&self, var: &str) -> Option<ReturnTypes> {
        self.current
            .borrow()
            .get(var)
            .cloned()
            .or_else(|| self.parent.as_ref()?.get(var))
    }

    pub fn set(&self, var: String, value: ReturnTypes) {
        self.current.borrow_mut().insert(var, value);
    }
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        Scope {
            parent: Some(Rc::new(Scope {
                parent: self.parent.clone(),
                current: self.current.clone(),
            })),
            current: Default::default(),
        }
    }
}
