use std::cell::RefCell;
use std::{env, fs};
use std::rc::Rc;

use monkey::lexer::lexer::Lexer;
use monkey::object::env::Env;
use monkey::parser::parser::Parser;
use monkey::eval::eval::Eval;

fn main() {
    
    let args:Vec<String> = env::args().collect();
    let input;
    if args.len() >= 2 {
        let filename = &args[1];
        let file_val = fs::read_to_string(filename);
        match file_val {
            Ok(s) => {
                input = s;
            }
            Err(e) => {
                println!("Error: {}", e);
                return 
            }
        }
    } else {
        println!("No File To Read");
        return 
    }

    let lexer = Lexer::new(input);
        
    let mut parser = Parser::new(lexer);
    let stmt = parser.parse_program();
    let env = Env::new();
    let mut eval = Eval::new(Rc::new(RefCell::new(env)));
    eval.eval_program(stmt);
}
