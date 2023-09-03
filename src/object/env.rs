use std::collections::HashMap;

use super::object::Object;

#[derive(Clone)]
pub struct Env {
    map: HashMap<String, Object<'static>>,
    outer: Option<Box<Env>>,
}
impl Env {
    pub fn new(outer: Option<Env>) -> Self{
        let outer = match outer {
            Some(e) => {
                Some(Box::new(e))
            }
            None => {
                None
            }
        };
        return Env {
            map: HashMap::new(),
            outer,
        }
    }
    pub fn read_ident(&self, s: String) -> Option<&Object> {
        let obj = self.map.get(&s.clone());
        if obj.is_some() {
            return obj
        }
        let mut outer = &self.outer;

        loop {
            match outer {
                Some(b) =>  {
                    let temp = b.read_ident(s.clone());
                    if temp.is_some() {
                        return Some(temp.unwrap())
                    }

                    outer = &b.outer;

                }
                None => {
                    return None;
                }
            }
        }
    }
    pub fn add_ident(&mut self, val: Object<'static>, key: String) {
        self.map.insert(key, val);
    }
}
