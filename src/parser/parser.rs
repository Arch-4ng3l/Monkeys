use crate::lexer::lexer::Lexer;
use crate::token::token::Token;
use crate::ast::ast::*;
//use crate::Lexer;



pub struct Parser {
    cur_token: Token,
    next_token: Token,
    lexer: Lexer,
}

impl Parser {
    pub fn new(l: Lexer) -> Self{

        let mut p = Parser{
            cur_token: Token::None,
            next_token: Token::None,
            lexer: l,
        };
        p.next();

        return p
    }

    pub fn parse_program(&mut self) -> Vec<Statement>{
        let mut v: Vec<Statement> = Vec::new();

        while self.cur_token != Token::EOF {
            let stmt = self.parse_statement();        
            println!("Statement: {}", stmt);
            v.push(stmt);
            self.next();
        }

        return v;
    }

    fn parse_statement(&mut self) -> Statement {
        match self.cur_token {
            Token::Var => {
                return self.parse_var();
            }
            _ => {
                return Statement::None;
            }
        }
    }
    fn parse_var(&mut self) -> Statement {
        let token = self.next_token.clone();
        match  token {
            Token::Ident(s) => {
                let ident = s;
                self.next();
                if self.next_token != Token::Assign {
                    return Statement::None;
                }
                self.next();
                self.next();
                println!("Var Cur Token: {:?}", self.cur_token);
                let val = self.parse_expression(Precedences::Lowest);
                println!("Var Value: {:?}", val);
                
                return Statement::Var(Ident{literal: ident.to_string()}, val)
            }
            _ => {

            }

        }
        return Statement::None
    }

    fn parse_expression(&mut self, precedence: Precedences) -> Expression {
        println!("Expression Current Token: {:?}" ,self.cur_token);

        let mut left = Expression::None;
        match self.cur_token {
            Token::Minus => {
                self.next();
                let exp =  self.parse_expression(Precedences::Prefix);
                left = Expression::Prefix(Prefix::Minus, Box::new(exp));
                println!("Done {:?}", left);

                //self.next();
            }
            _ => {
            }
        }

        match self.cur_token {
            Token::Int(i) => {
                match left {
                    Expression::None =>  {
                        left = Expression::Literal(
                            Literals::Int(
                                i
                            )
                        );
                    }
                    _ => {

                    }

                }
                while precedence < Self::token_to_precedence(self.next_token.clone()) {
                    println!("Cur: {:?} | Next Precidence{:?}", precedence,Self::token_to_precedence(self.next_token.clone()));
                    self.next();
                    if self.cur_token == Token::EOF || self.cur_token == Token::EOF{
                        break;
                    }

                    left = self.get_infix_fn(left);
                    //self.next();
                }
                return left;

                
            }
            Token::Bool (b) => {
                return Expression::Literal(
                    Literals::Bool(
                        b 
                    )
                )
            }


            _ => {
            }
        }

        println!("Token : {:?}", self.cur_token);
        return Expression::None;
    }

    fn token_to_precedence(token: Token) -> Precedences {
        let precedence = match token {
            Token::Minus | Token::Plus => {
                Precedences::Sum
            }
            Token::Star | Token::Slash => {
                Precedences::Product
            }
            _ => {
                Precedences::Lowest
            }
        };
        return precedence
    }

    
    fn next(&mut self) {
        self.cur_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn get_infix_fn(&mut self, exp: Expression) -> Expression {
        println!("Token: {:?}, Expression: {:?}", self.cur_token, exp);
        match self.cur_token {
            Token::Plus => {
                self.next();
                return Expression::Infix(Infix::Plus,
                    Box::new(exp),
                    Box::new(self.parse_expression(Precedences::Sum))
                );
            }
            Token::Minus => {
                self.next();
                return Expression::Infix(Infix::Minus, Box::new(exp), Box::new(self.parse_expression(Precedences::Sum)));
            }
            Token::Star => {
                self.next();
                return Expression::Infix(Infix::Star,
                    Box::new(exp),
                    Box::new(self.parse_expression(Precedences::Product))
                );
            }
            Token::Slash => {
                self.next();
                return Expression::Infix(Infix::Slash, Box::new(exp), Box::new(self.parse_expression(Precedences::Product)));
            }
            _ => {
                return Expression::None;
            }
        }
    }

    fn parse_prefix(&mut self) -> Expression{
        match self.cur_token {
            Token::Minus => {
                return Expression::Prefix(Prefix::Minus, Box::new(self.parse_expression(Precedences::Prefix)));
            }
            _ => {
                return Expression::None;
            }
        }
    }

}



