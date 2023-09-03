#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Token {
    Assign,


    Ident(String),
    Int(i64),
    Bool(bool),
    String(String),

    Comma,

    Plus,
    Minus,
    Star,
    Slash,
    EQ, 
    NotEQ,
    GT, 
    LT,

    LPAREN,
    RPAREN,   
    LBRACE,  
    RBRACE,  
    LBRACKET,
    RBRACKET,

    If, 
    Else,
    Func,
    Var,
    Return,

    NewLine,

    Illegal,
    EOF,

    None,
}
