use monkey::lexer::lexer::Lexer;
use monkey::parser::parser::Parser;

fn main() {
    //let input = "if ( 10 < 10 ) { \n var x = -10 * 29 }".to_string();
    let lexer = Lexer::new("-10 * 2".to_string());
        
    let mut parser = Parser::new(lexer);
    let stmt = parser.parse_program();
    for i in stmt {
        println!("I : ====== {:?}", i);
    }
}
