use core::fmt;
use std::{rc::Rc, cell::RefCell};

use crate::ast::ast::{BlockStmt, Ident};

use super::env::Env;

pub type BuiltinFunction = fn(Vec<Object>) -> Object;

#[derive(Clone)]
pub enum Object {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Arr(Vec<Object>),
    Return(Box<Object>),

    Function(Vec<Ident>, BlockStmt, Rc<RefCell<Env>>),
    Builtin(i64, BuiltinFunction),

    Error(String),
    None,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(i) => {
                write!(f, "{}", i)
            }
            Object::Bool(b) => {
                write!(f, "{}", b)
            }
            Object::String(s) => {
                write!(f, "{}", s)
            }
            Object::Float(fl) => {
                write!(f, "{}", fl)
            }
            Object::Function(params, block, _) => {
                write!(f, "func({:?}) {:?}", params, block)
            }
            Object::Arr(arr) => {
                let objs = arr
                    .iter()
                    .map(|obj| format!("{}", obj))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string();
                write!(f, "[{}]", objs)
            }
            _ =>{
                write!(f, "")
            }
        }
    }
}
