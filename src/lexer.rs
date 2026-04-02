use crate::token::{self, TokenType};

// In Rust, we use u8 to represent a byte.
// u8 is unsigned 8 bit.
// It stores numbers from 0 to 2^8-1, which equals 0 to 255,
// which inludes all ASCII characters.
pub struct Lexer {
    input: Vec<u8>,
    position: usize, // 指向所输入字符串中与ch字节对应的字符
    read_position: usize, // 始终指向所输入字符串中的下一个字符
    ch: u8, // The char pointed by position
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
            b':' => new_token(TokenType::Colon, self.ch),
            b'(' => new_token(TokenType::LParen, self.ch),
            b')' => new_token(TokenType::RParen, self.ch),
            b'{' => new_token(TokenType::LBrace, self.ch),
            b'}' => new_token(TokenType::RBrace, self.ch),
            b'[' => new_token(TokenType::LBracket, self.ch),
            b']' => new_token(TokenType::RBracket, self.ch),
            b'"' => {
                let literal = self.read_string();
                token::Token {
                    token_type: TokenType::String, 
                    literal,
                }
            }
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
        // Stops after the identifier.
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn skip_whitespace(&mut self) {
        // Move position to the first non-whitespace character
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        // Move position to the first non-digit character
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn read_string(&mut self) -> String {
        // Move position one step after starting`"`
        self.read_char();
        let position = self.position;
        while self.ch != b'"' && self.ch != 0 {
            self.read_char();
        }
        // Now char points to ending `"`.
        // We could slice between "..."
        // Caller must move to next char
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

#[cfg(test)]
mod tests;
