use monkey::lexer::lexer::Lexer;
use monkey::parser::parser::Parser;

fn main() {
    let input = "if ( 10 < 10 ) { \n var x = -10 * 29 } else { var y = 19; }".to_string();
    let lexer = Lexer::new(input);
        
    let mut parser = Parser::new(lexer);
    let stmt = parser.parse_program();
    for i in stmt {
        println!("I : ====== {:?}", i);
    }
}
