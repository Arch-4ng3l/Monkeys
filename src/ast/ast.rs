use std::fmt;


#[derive(Debug)]
pub struct Ident {
    pub literal: String,
}
impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}



#[derive(Debug)]
pub enum Prefix {
    Minus, 
    None
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-")
    }
}

#[derive(Debug)]
pub enum Infix {
    Plus, 
    Minus,
    Star,
    Slash,

    None
}
impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Infix::Plus => {
                write!(f , "+")
            }
            Infix::Minus => {
                write!(f, "-")
            }
            Infix::Star => {
                write!(f, "*")
            }
            Infix::Slash => {
                write!(f, "/")
            }
            _ => {
                write!(f, "")
            }

        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Ident(Ident),
    Literal(Literals),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    None,
}

impl fmt::Display for Expression{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Ident(i) => {
                write!(f, "{}", i)
            }
            Expression::Prefix(p, exp) => {
                write!(f, "({}{})", p, *exp)
                
            }
            Expression::Infix(i,left ,right ) => {
                write!(f, "({}{}{})", left, i, right)
            }
            Expression::Literal(l) => {
                write!(f, "{}", l)
            }
            _ => {
                write!(f, "")
            }
        }
    }
}


#[derive(Debug)]
pub enum Statement {
    Var(Ident, Expression),
    Return(Expression),
    ExpressionStmt(Expression),

    None,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Var(ident, exp) => {
                write!(f, "var {} = {}", ident, exp)
            }
            _ => {
                write!(f, "")
            }
        }
    }
}

#[derive(Debug)]
pub enum Literals {
    Int(i64),
    Bool(bool),
}
impl fmt::Display for Literals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literals::Int(i) => {
                write!(f, "{}", i)
            }
            Literals::Bool(b) => {
                write!(f, "{}", b)
            }
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Precedences {
    Lowest,
    Equals,     
    LessGreater, 
    Sum,         
    Product,    
    Prefix,    
    Call,     
    Index,   
}

