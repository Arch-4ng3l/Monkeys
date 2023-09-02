#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Token {
    Assign,


    Ident(String),
    Int(i64),
    Bool(bool),

    Plus,
    Minus,
    Star,
    Slash,
    
    Func,
    Var,
    Return,

    NewLine,

    Illegal,
    EOF,

    None,
}
