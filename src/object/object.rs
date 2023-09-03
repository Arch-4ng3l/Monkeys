use core::fmt;

use crate::ast::ast::{BlockStmt, Ident};

use super::env::Env;

#[derive(Clone)]
pub enum Object<'a > {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Return(Box<Object<'a>>),

    Function(Vec<Ident>, BlockStmt, Box<&'a Env>),
    None,
}

impl fmt::Display for Object<'_> {
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
            _ =>{
                write!(f, "")
            }
        }
    }
}
