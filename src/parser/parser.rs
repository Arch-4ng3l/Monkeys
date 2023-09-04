use crate::lexer::lexer::Lexer;
use crate::token::token::Token;
use crate::ast::ast::*;

pub struct Parser {
    cur_token: Token,
    next_token: Token,
    lexer: Lexer,
}

impl Parser {
    pub fn new(l: Lexer) -> Self{

        let mut parser = Parser{
            cur_token: Token::None,
            next_token: Token::None,
            lexer: l,
        };
        parser.next();
        parser.next();

        parser
    }

    pub fn parse_program(&mut self) -> Program{
        let mut parsed: Vec<Statement> = Vec::new();

        while self.cur_token != Token::EOF {
            let stmt = self.parse_statement();        
            parsed.push(stmt);
            self.next();
        }

        parsed
    }
    fn parse_block(&mut self) -> BlockStmt {
        let mut parsed = Vec::new();

        if self.cur_token != Token::LBRACE {
            return parsed
        }

        self.next();
        while self.cur_token != Token::RBRACE {
            parsed.push(self.parse_statement()); 
            self.next();
        }

        parsed
    }

    fn parse_statement(&mut self) -> Statement {
        return match self.cur_token {
            Token::Var => {
                self.parse_var()
            }
            Token::Return => {
                self.next();
                Statement::Return(self.parse_expression(Precedences::Lowest))
            }
            _ => {
                Statement::ExpressionStmt(self.parse_expression(Precedences::Lowest))
            }
        }
    }
    fn parse_var(&mut self) -> Statement {
        let token = self.next_token.clone();
        return match  token {
            Token::Ident(s) => {
                let ident = s;
                self.next();
                if self.next_token != Token::Assign {
                    return Statement::None
                }
                self.next();
                self.next();
                let val = self.parse_expression(Precedences::Lowest);
                
                Statement::Var(Ident{literal: ident.to_string()}, val)
            }
            _ => {
            Statement::None

            }
        }

    }

    fn parse_expression(&mut self, precedence: Precedences) -> Expression {
        let mut left = self.parse_prefix();
        if left == Expression::None  {
            match &self.cur_token {
                Token::Int(i) => { left = Expression::Literal(Literals::Int(*i)) }
                Token::Bool(b) => { left = Expression::Literal(Literals::Bool(*b)) }
                Token::String(s) => { left = Expression::Literal(Literals::String(s.clone())) }
                Token::Ident(i) => { left = Expression::Ident(Ident{literal: i.clone()}) }
                _ => { }
            }
        }

        while precedence < Self::token_to_precedence(self.next_token.clone()) {
            self.next();
            if self.cur_token == Token::EOF || self.cur_token == Token::EOF{
                break;
            }

            left = self.parse_infix(left);
        }
            
        left
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
            Token::LPAREN => {
                Precedences::Call
            }
            Token::LBRACKET => {
                Precedences::Index
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
                    return Expression::None
                }
                self.next();
                let cond = self.parse_expression(Precedences::Lowest);

                if self.next_token != Token::RPAREN {
                    return Expression::None
                }
                self.next();

                self.next();
                let if_block = self.parse_block();
                self.next();

                let else_block;
                if self.cur_token == Token::Else {
                    self.next();
                    else_block = Some(self.parse_block());
                } else {
                    else_block = None;
                }

                Expression::If(
                    Box::new(cond),
                    if_block,
                    else_block
                )
            }
            Token::Func => {
                self.next();
                let params = self.parse_function_params();

                self.next();

                let body = self.parse_block();

                Expression::Function(
                    params, 
                    body,
                )
            }

            Token::LBRACKET => {
                self.next();
                println!("bracket");
                let mut exps = Vec::new();
                while self.cur_token != Token::RBRACKET {
                    exps.push(self.parse_expression(Precedences::Lowest));
                    self.next();
                    if self.cur_token == Token::Comma {
                        self.next();
                    }
                }

                Expression::Literal(Literals::Arr(exps))
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
        return match self.cur_token {
            Token::LPAREN => {
                let args = self.parse_expression_list();

                Expression::FunctionCall(args, Box::new(exp))

            }
            Token::LBRACKET => {
                self.next();
                let ind = self.parse_expression(Precedences::Index);
                self.next();
                Expression::Index(Box::new(exp), Box::new(ind))
            }
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
    fn parse_function_params(&mut self) -> Vec<Ident> {
        let mut idents = Vec::new();
    
        self.next();
        match &self.cur_token {
            Token::Ident(i) => {
                idents.push(Ident { literal: i.to_string() });
                self.next();
            }
            _ => {
                return idents
            }
        }
        if self.cur_token == Token::RPAREN {
            return idents
        }

        self.next();

        while self.next_token == Token::Comma {
            match &self.cur_token {
                Token::Ident(i) => {
                    idents.push(Ident{literal: i.to_string()});
                    self.next();
                    self.next();
                }
                _ => {}
            }

        }
        idents
        
    }

    fn parse_expression_list(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();
        if self.cur_token != Token::LPAREN {
            return args;
        }
        self.next();
        args.push(self.parse_expression(Precedences::Lowest));

        while self.next_token == Token::Comma {
            self.next();
            self.next();
            args.push(self.parse_expression(Precedences::Lowest));
        }

        args
    }

}



