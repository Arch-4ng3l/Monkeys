use monkey::lexer::lexer::Lexer;
use monkey::parser::parser::Parser;
use monkey::eval::eval::Eval;

fn main() {
    //let input = "if ( 10 < 10 ) { \n var x = -10 * 29 } else { var y = 19; }".to_string();
    let input = "var x = func(y, z) { return x * y } \n if (10 < 20) { return x(10, 10) }".to_string();
    let lexer = Lexer::new(input);
        
    let mut parser = Parser::new(lexer);
    let stmt = parser.parse_program();
    let mut eval = Eval::new();
    eval.eval_program(stmt);
}
