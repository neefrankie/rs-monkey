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

        match self.ch {
            b'=' => {
                // = could be assignment or comparison
                if self.peek_char() == b'=' {
                    let first = self.ch;
                    return self.emit_double_char_token(
                        TokenType::Eq, 
                        first,
                    );
                } else {
                    return self.emit_single_char_token(TokenType::Assign);
                }
            },
            b'+' => {
                return self.emit_single_char_token(TokenType::Plus);
            },
            b'-' => {
                return self.emit_single_char_token(TokenType::Minus);
            },
            b'!' => {
                if self.peek_char() == b'=' {
                    let first = self.ch;
                    return self.emit_double_char_token(
                        TokenType::NotEq, 
                        first
                    );
                } else {
                    return self.emit_single_char_token(TokenType::Bang)
                }
            },
            b'/' => {
                return self.emit_single_char_token(TokenType::Slash);
            },
            b'*' => {
                return self.emit_single_char_token(TokenType::Asterisk);
            },
            b'<' => {
                return self.emit_single_char_token(TokenType::LessThan);
            },
            b'>' => {
                return self.emit_single_char_token(TokenType::GreaterThan);
            },
            b';' => {
                return self.emit_single_char_token(TokenType::Semicolon);
            },
            b',' => {
                return self.emit_single_char_token(TokenType::Comma);
            },
            b':' => {
                return self.emit_single_char_token(TokenType::Colon);
            },
            b'(' => {
                return self.emit_single_char_token(TokenType::LParen);
            },
            b')' => {
                return self.emit_single_char_token(TokenType::RParen);
            },
            b'{' => {
                return self.emit_single_char_token(TokenType::LBrace);
            },
            b'}' => {
                return self.emit_single_char_token(TokenType::RBrace);
            },
            b'[' => {
                return self.emit_single_char_token(TokenType::LBracket);
            },
            b']' => {
                return self.emit_single_char_token(TokenType::RBracket);
            },
            b'"' => {
                let literal = self.read_string();
                return token::Token {
                    token_type: TokenType::String, 
                    literal,
                }
            }
            0 => {
                return token::Token {
                    token_type: TokenType::Eof,
                    literal: String::new()
                };
            },
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = token::lookup_ident(&literal);
                    return token::Token {
                        token_type,
                        literal
                    };
                } if is_digit(self.ch) {
                    let literal = self.read_number();
                    return token::Token {
                        token_type: TokenType::Int,
                        literal
                    };
                } else {
                    return self.emit_single_char_token(TokenType::Illegal);
                }
            },
        }
    }

    fn emit_single_char_token(&mut self, token_type: TokenType) -> token::Token {
        let ch = self.ch;
        // Move position to the char after current token.
        self.read_char();
       
        token::Token {
            token_type: token_type,
            literal: char::from(ch).to_string(),
        }
    }

    fn emit_double_char_token(&mut self, token_type: TokenType, first: u8) -> token::Token {
        self.read_char(); // 读取第二个字符
        let second = self.ch;
        self.read_char(); // 移动到 token 之后
        token::Token {
            token_type,
            literal: format!("{}{}", char::from(first), char::from(second)),
        }
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
        // Move to string start. Skip starting quote.
        self.read_char();
        let start = self.position;
        while self.ch != b'"' && self.ch != 0 {
            self.read_char();
        }
        // Now char points to ending `"`.
        // For example, slicing `"hello"  starts at 1 and ends at 6.
        let end = self.position;
        // Skip ending quote.
        self.read_char();
        
        String::from_utf8_lossy(&self.input[start..end]).to_string()
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
