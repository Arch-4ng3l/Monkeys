use crate::token::token::Token;

pub struct Lexer {
    pos: usize, 
    next_pos: usize,
    ch: u8, 
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer{
            pos: 0, 
            next_pos: 0,
            ch: 0, 
            input: input.into_bytes(),
        };
        l.read_char();
        return l;

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
        let tok: Token;
        self.skip_whitespace();
        match self.ch {
            b'=' => {
                tok = Token::Assign
            }
            b'+' => {
                tok = Token::Plus
            }
            b'-' => {
                tok = Token::Minus
            }
            b'*' => {
                tok = Token::Star
            }
            b'/' => {
                tok = Token::Slash
            }
            b'>' => {
                tok = Token::GT
            }
            b'<' => {
                tok = Token::LT
            }
            b'\n' => {
                tok = Token::NewLine
            }
            b'(' => {
                tok = Token::LPAREN
            }
            b')' => {
                tok = Token::RPAREN
            }
            b'{' => {
                tok = Token::LBRACE
            }
            b'}' => {
                tok = Token::RBRACE
            }

            0 => {
                tok = Token::EOF
            }
            _ => {
                if is_char(self.ch) {
                    let val = self.read_ident();
                    match val.as_str() {
                        "func" => {
                            tok = Token::Func
                        }
                        "var" => {

                            tok = Token::Var
                        }
                        "true" => {
                            tok = Token::Bool(true)
                        }
                        "false" => {
                            tok = Token::Bool(false)
                        }
                        "return" => {
                            tok = Token::Return
                        }
                        "if" => {
                            println!("if");
                            tok = Token::If
                        }
                        "else" => {
                            tok = Token::Else
                        }
                        _ => {
                            tok = Token::Ident(val)
                        }
                    }
                } else if is_digit(self.ch) {
                    let val = self.read_number();

                    tok = Token::Int(val);
                } else {
                    tok = Token::Illegal
                }
            }
        };

        self.read_char();

        return tok;
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

        return str;
    }
    fn read_number(&mut self) -> i64 {
        let mut str = String::new(); 
        while is_digit(self.ch) {
            str = str + &(self.ch as char).to_string(); 

            self.read_char(); 
        }
        return str.parse().unwrap();
    }

}

fn is_char(ch: u8) -> bool {
    return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z';
}

fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9'; 
}
