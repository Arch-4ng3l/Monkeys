pub mod lexer;
use crate::lexer::lexer::*;
pub mod parser;
use crate::parser::parser::*;
pub mod token;
pub mod ast;

fn main() {
    let lexer = Lexer::new("var x = -19 * 2 + 39 * 3".to_string());
    
    let mut parser = Parser::new(lexer);
    parser.parse_program();
}
