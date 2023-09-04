use std::cell::RefCell;
use std::rc::Rc;

use monkey::lexer::lexer::Lexer;
use monkey::object::env::Env;
use monkey::parser::parser::Parser;
use monkey::eval::eval::Eval;

fn main() {
    let input = "var f = [1, 2, 3] \n f[1]".to_string();
    let lexer = Lexer::new(input);
        
    let mut parser = Parser::new(lexer);
    let stmt = parser.parse_program();
    let env = Env::new();
    let mut eval = Eval::new(Rc::new(RefCell::new(env)));
    eval.eval_program(stmt);
}
