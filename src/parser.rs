use std::vec::Vec;

use crate::ast;
use crate::lexer;
use crate::token::{self, TokenType};

mod precedence;
mod statments;
mod expressions;
mod errors;

use errors::ParseError;
use precedence::{token_precedence};


pub struct Parser {
    lexer: lexer::Lexer,
    current_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    /// Creates a new parser.
    /// Initializes `current_token` and `peek_token` by reading the first two tokens.
    pub fn new(mut lexer: lexer::Lexer) -> Self {
        let current_token = lexer.next_token();
        println!("New Parser: current_token: {:?}", current_token);

        let peek_token = lexer.next_token();
        println!("New Parser: peek_token: {:?}\n", peek_token);

        let parser = Parser {
            lexer: lexer,
            current_token: current_token,
            peek_token,
        };

        
        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        println!("Parser: \n\tcurrent_token: {:?}, \n\tpeek_token: {:?}\n", self.current_token, self.peek_token)
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Vec<ParseError>> { 
        let mut statements: Vec<Box<dyn ast::Statement>> = Vec::new();
        let mut errors = Vec::new();

        while self.current_token.token_type != TokenType::Eof {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => errors.push(e),
            }
            self.next_token();
        }

        if errors.is_empty() {
            Ok(ast::Program { statements })
        } else {
            Err(errors)
        }
    }


    fn peek_precedence(&self) -> i32 {
        return token_precedence(self.peek_token.token_type)
    }

    fn current_precedence(&self) -> i32 {
        return token_precedence(self.current_token.token_type)
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> Result<(), ParseError> {
        if self.peek_token_is(token_type) {
            self.next_token();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: TokenType::Assign, 
                got: self.current_token.token_type,
            })
        }
    }

}

#[cfg(test)]
mod tests;