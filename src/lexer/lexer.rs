use crate::token::token::Token;

pub struct Lexer {
    pos: usize, 
    next_pos: usize,
    ch: u8, 
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(mut input: String) -> Lexer {
        input.push('\0');
        let mut lexer = Lexer{
            pos: 0, 
            next_pos: 0,
            ch: 0, 
            input: input.into_bytes(),
        };
        lexer.read_char();

        lexer

    }

    pub fn read_char(&mut self) {
        if self.next_pos < self.input.len() {
            self.ch = self.input[self.next_pos];
            self.pos = self.next_pos;
            self.next_pos += 1;
        } else {
            self.ch = 0;
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            b'=' => {
                Token::Assign
            }
            b'+' => {
                Token::Plus
            }
            b'-' => {
                Token::Minus
            }
            b'*' => {
                Token::Star
            }
            b'/' => {
                Token::Slash
            }
            b'>' => {
                Token::GT
            }
            b'<' => {
                Token::LT
            }
            b'\n' => {
                Token::NewLine
            }
            b'(' => {
                Token::LPAREN
            }
            b')' => {
                Token::RPAREN
            }
            b'{' => {
                Token::LBRACE
            }
            b'}' => {
                Token::RBRACE
            }
            b',' => {
                Token::Comma
            }
            b'[' => {
                Token::LBRACKET
            }
            b']' => {
                Token::RBRACKET
            }
            b'"' => {
                let val = self.read_str();
                Token::String(val)
            }

            0 => {
                Token::EOF
            }
            _ => {
                if is_char(self.ch) {
                    let val = self.read_ident();
                    match val.as_str() {
                        "func" => {
                            Token::Func
                        }
                        "var" => {
                            Token::Var
                        }
                        "true" => {
                            Token::Bool(true)
                        }
                        "false" => {
                            Token::Bool(false)
                        }
                        "return" => {
                            Token::Return
                        }
                        "if" => {
                            Token::If
                        }
                        "else" => {
                            Token::Else
                        }
                        _ => {
                            Token::Ident(val)
                        }
                    }
                } else if is_digit(self.ch) {
                    Token::Int(self.read_number())
                } else {
                    Token::Illegal
                }
            }
        };
        
        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn read_ident(&mut self) -> String {
        let mut str = String::new();
        while is_char(self.ch) {
            str = str + &(self.ch as char).to_string();
            self.read_char(); 
        }
        self.back();

        str
    }
    fn read_number(&mut self) -> i64 {
        let mut str = String::new(); 
        while is_digit(self.ch) {
            str = str + &(self.ch as char).to_string(); 

            self.read_char(); 
        }
        self.back();

        str.parse().unwrap()
    }
    fn read_str(&mut self) -> String {
        let mut str = String::new();
        self.read_char();
        while self.ch != b'"' {
            str = str + &(self.ch as char).to_string();
            self.read_char();
        }
        str
    }

    fn back(&mut self) {
        if self.next_pos > 0 {
            self.pos -= 1;
            self.next_pos -= 1;
            self.ch = self.input[self.pos];
        } else {
            self.ch = 0;
        }
    }

}


fn is_char(ch: u8) -> bool {
    return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z';
}

fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9'; 
}
