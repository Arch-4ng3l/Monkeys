pub mod lexer;
pub mod parser;
pub mod token;
pub mod ast;



#[cfg(test)]
mod tests{
    use crate::{lexer::lexer::Lexer, parser::parser::Parser, ast::ast::{Statement, Ident, Expression, Literals, Infix, Prefix}};

#[test]
    fn lexer() {
        let input = "var x = 10\n var y = true\n var z = -19 + 29\n var a = 10 < 20";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let stmts = parser.parse_program();
        let res = vec![
            Statement::Var(
            Ident{literal: "x".to_string()},
            Expression::Literal(Literals::Int(10))
            ),
            Statement::Var(
            Ident { literal: "y".to_string() },
            Expression::Literal(Literals::Bool(true))
            ),
            Statement::Var(
            Ident { literal: "z".to_string() },
            Expression::Infix(
            Infix::Plus,
            Box::new(Expression::Prefix(Prefix::Minus, 
            Box::new(Expression::Literal(Literals::Int(19))))), 
            Box::new(Expression::Literal(Literals::Int(29))))
            ),
            Statement::Var(
            Ident { literal: "a".to_string() },
            Expression::Infix(
            Infix::LT, 
            Box::new(Expression::Literal(Literals::Int(10))),
            Box::new(Expression::Literal(Literals::Int(20))),
            ))


        ];
        
        for i in 0..stmts.len() {
            assert_eq!(stmts[i], res[i]);
        }
    }   
}
