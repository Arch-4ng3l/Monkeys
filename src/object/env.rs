use std::{collections::HashMap, cell::RefCell , rc::Rc};

use super::object::Object;

#[derive(Clone)]
pub struct Env {
    map: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Env>>>,
}
impl Env {
    pub fn new() -> Self{
        return Env {
            map: HashMap::new(),
            outer: None,
        }
    }
    pub fn new_with_outer(env: Rc<RefCell<Env>>) -> Self {
        return Env {
            map: HashMap::new(),
            outer: Some(env)
        }
    }
    pub fn read_ident(&self, s: &str) -> Option<Object> {
        match self.map.get(&s.to_string()) {
            Some(obj) => {
                Some(obj.clone())
            }
            None => {
                match self.outer {
                    Some(ref outer) => {
                        outer.borrow_mut().read_ident(s)
                    }
                    None => {
                        None
                    }
                }
            }
        }
    }
    pub fn add_ident(&mut self, val: Object, key: String) {
        self.map.insert(key, val);
    }
}
