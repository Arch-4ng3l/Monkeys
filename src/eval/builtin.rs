use std::collections::HashMap;
use crate::object::object::Object;



pub fn new_builtin_functions() -> HashMap<String, Object> {
    let mut map = HashMap::new();
    map.insert("len".to_string(), Object::Builtin(1, len));
    map.insert("print".to_string(), Object::Builtin(1, print));

    map
}

fn len(args: Vec<Object>) -> Object {
    
    match &args[0] {
        Object::Arr(arr) => {
            Object::Int(arr.len() as i64)
        }
        Object::String(str) => {
            Object::Int(str.len() as i64)
        }
        _ => {
            Object::Error(format!("{} Doesnt Have A Length Property", args[0]))
        }
    }
}

fn print(args: Vec<Object>) -> Object {
    println!("{}", args[0]);
    Object::None
}


