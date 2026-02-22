use crate::token::{self, TokenType};

pub struct Lexer {
    input: Vec<u8>,
    position: usize, // 指向所输入字符串中与ch字节对应的字符
    read_position: usize, // 始终指向所输入字符串中的下一个字符
    ch: u8,
}

impl Lexer { 
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char(); // Move to first character
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // 或者用 b'\0'
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
        
        println!("LEXER: pos={}, read_pos={}, ch='{}'", self.position, self.read_position, self.ch as char)
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = char::from(ch).to_string() + &char::from(self.ch).to_string();
                    token::Token{ token_type: TokenType::Eq, literal }
                } else {
                    new_token(TokenType::Assign, self.ch)
                }
            },
            b'+' => new_token(TokenType::Plus, self.ch),
            b'-' => new_token(TokenType::Minus, self.ch),
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = char::from(ch).to_string() + &char::from(self.ch).to_string();
                    token::Token{ token_type: TokenType::NotEq, literal }
                } else {
                    new_token(TokenType::Bang, self.ch)
                }
            },
            b'/' => new_token(TokenType::Slash, self.ch),
            b'*' => new_token(TokenType::Asterisk, self.ch),
            b'<' => new_token(TokenType::LessThan, self.ch),
            b'>' => new_token(TokenType::GreaterThan, self.ch),
            b';' => new_token(TokenType::Semicolon, self.ch),
            b',' => new_token(TokenType::Comma, self.ch),
            b'(' => new_token(TokenType::LParen, self.ch),
            b')' => new_token(TokenType::RParen, self.ch),
            b'{' => new_token(TokenType::LBrace, self.ch),
            b'}' => new_token(TokenType::RBrace, self.ch),
            0 => token::Token { token_type: TokenType::Eof, literal: String::new() },
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = token::lookup_ident(&literal);
                    // 提前退出
                    return token::Token { token_type, literal };
                } if is_digit(self.ch) {
                    let literal = self.read_number();
                    return token::Token { token_type: TokenType::Int, literal };
                } else {
                    new_token(TokenType::Illegal, self.ch)
                }
                
            },
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    
}

fn new_token(kind: token::TokenType, ch: u8) -> token::Token {
    token::Token {
        token_type: kind,
        literal: char::from(ch).to_string(),
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit()
}


mod tests {
    
    #[test]
    fn test_next_token() {
        use super::*;

        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
	x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
"#;

        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::LessThan, "<"),
            (TokenType::Int, "10"),
            (TokenType::GreaterThan, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::LessThan, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (expected_type, expected_literal) in tests {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal.to_string());
        }
    }
}
