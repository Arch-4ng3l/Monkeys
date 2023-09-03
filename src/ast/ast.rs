use std::fmt;

pub type BlockStmt = Vec<Statement>;

pub type Program = BlockStmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ident {
    pub literal: String,
}
impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Prefix {
    Minus, 
    None
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Infix {
    Plus, 
    Minus,
    Star,
    Slash,
    GT, 
    LT,
    EQ, 
    NotEQ,

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Ident(Ident),
    Literal(Literals),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    If(
        Box<Expression>,
        BlockStmt,
        Option<BlockStmt>,
    ),
    Function(
        Vec<Ident>, 
        BlockStmt,
    ),

    FunctionCall(
        Vec<Expression>,
        Box<Expression>,
    ),
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
            Expression::If(cond, i, e) => {
                match e {
                    Some(exp) =>{
                        write!(f, "if ({}) ", cond);
                        for s in i {
                            write!(f, "{}", s);
                        }
                        write!(f, "else ");
                        for s in exp {
                            write!(f, "{}", s);
                        }
                        write!(f, "")



                    }
                    None => {
                        write!(f, "if ({}) ", cond);
                        for s in i {
                            write!(f, "{}", s);
                        }
                        write!(f, "")

                    }
                }
            }
            _ => {
                write!(f, "")
            }
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literals {
    Int(i64),
    Bool(bool),
    String(String)
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
            Literals::String(s) => {
                write!(f, "{}", s)
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

