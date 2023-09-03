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
        p.next();

        return p
    }

    pub fn parse_program(&mut self) -> Vec<Statement>{
        let mut v: Vec<Statement> = Vec::new();

        while self.cur_token != Token::EOF {
            let stmt = self.parse_statement();        
            v.push(stmt);
            println!("New");
            self.next();
        }

        return v;
    }
    fn parse_block(&mut self) -> Vec<Statement> {
        let mut v = Vec::new();
        if self.cur_token != Token::LBRACE {
            return v;
        }
        self.next();
        while self.cur_token != Token::RBRACE {
            v.push(self.parse_statement()); 
            self.next();
        }

        v
    }

    fn parse_statement(&mut self) -> Statement {
        match self.cur_token {
            Token::Var => {
                return self.parse_var();
            }
            _ => {
                return Statement::ExpressionStmt(self.parse_expression(Precedences::Lowest));
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
                let val = self.parse_expression(Precedences::Lowest);
                
                return Statement::Var(Ident{literal: ident.to_string()}, val)
            }
            _ => {

            }

        }
        return Statement::None
    }

    fn parse_expression(&mut self, precedence: Precedences) -> Expression {
        println!("Expression Current Token: {:?}" ,self.cur_token);

        let mut left = self.parse_prefix();

        if left == Expression::None  {
            match self.cur_token {
                Token::Int(i) => { left = Expression::Literal(Literals::Int(i)) }
                Token::Bool(b) => { left = Expression::Literal(Literals::Bool(b)) }
                _ => { }
            }


        }
        
        while precedence < Self::token_to_precedence(self.next_token.clone()) {
            println!("Cur: {:?} | Next Precidence{:?}", precedence,Self::token_to_precedence(self.next_token.clone()));
            self.next();
            if self.cur_token == Token::EOF || self.cur_token == Token::EOF{
                break;
            }

                left = self.parse_infix(left);
            }
            
        return left;
    }


    fn token_to_precedence(token: Token) -> Precedences {
        return match token {
            Token::Minus | Token::Plus => {
                Precedences::Sum
            }
            Token::Star | Token::Slash => {
                Precedences::Product
            }
            Token::EQ | Token::NotEQ => {
                Precedences::Equals
            }
            Token::LT | Token::GT => {
                Precedences::LessGreater
            }
            _ => {
                Precedences::Lowest
            }
        };
    }

    
    fn next(&mut self) {
        self.cur_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }
    fn parse_prefix(&mut self) -> Expression {

        match self.cur_token {
            Token::If => {
                self.next();
                if self.cur_token != Token::LPAREN {
                    println!("{:?}", self.cur_token);
                    return Expression::None
                }
                self.next();
                let cond = self.parse_expression(Precedences::Lowest);
                //self.next();
                if self.next_token != Token::RPAREN {
                    println!("BLOCK CURRENT {:?}", self.next_token);
                    return Expression::None
                }
                self.next();
                self.next();
                println!("BLOCK");
                let block = self.parse_block();
                println!("Block {:?}", block);
                println!("Block Current Token {:?}", self.cur_token);


                Expression::If(
                    Box::new(cond),
                    block,
                    if self.cur_token == Token::Else {
                        Some(self.parse_block())
                    } else {
                        None
                    }
                )
            }

            Token::Minus => {
                self.next();
                let exp =  self.parse_expression(Precedences::Prefix);
                return Expression::Prefix(Prefix::Minus, Box::new(exp));
            }
            _ => {
                return Expression::None
            }
        }
    }

    fn parse_infix(&mut self, exp: Expression) -> Expression {
        println!("infix");
        return match self.cur_token {
            Token::Plus => {
                self.next();
                Expression::Infix(
                    Infix::Plus,
                    Box::new(exp),
                    Box::new(self.parse_expression(Precedences::Sum))
                )
            }
            Token::Minus => {
                self.next();
                Expression::Infix(
                    Infix::Minus,
                    Box::new(exp),
                    Box::new(self.parse_expression(Precedences::Sum))
                )
            }
            Token::Star => {
                self.next();
                Expression::Infix(
                    Infix::Star,
                    Box::new(exp),
                    Box::new(self.parse_expression(Precedences::Product))
                )
            }
            Token::Slash => {
                self.next();
                Expression::Infix(
                    Infix::Slash,
                    Box::new(exp),
                    Box::new(self.parse_expression(Precedences::Product))
                )
            }
            Token::GT => {
                self.next(); 
                Expression::Infix(
                    Infix::GT,
                    Box::new(exp),
                    Box::new(self.parse_expression(Precedences::LessGreater))
                )
            }
            Token::LT => {
                self.next();
                Expression::Infix(
                    Infix::LT,
                    Box::new(exp),
                    Box::new(self.parse_expression(Precedences::LessGreater))
                )
            }
            _ => {
                Expression::None
            }
        }
    }


}



