use std::vec::Vec;

use crate::ast::{self, Statement};
use crate::lexer;
use crate::token::{self, TokenType};

mod precedence;
mod statements;
mod expressions;
mod errors;

use errors::ParseError;
use precedence::{Precedence};


pub struct Parser {
    lexer: lexer::Lexer,
    current_token: token::Token, // 输入中的当前此法单元
    peek_token: token::Token, // 下一个词法单元
}

impl Parser {
    /// Creates a new parser.
    /// Initializes `current_token` and `peek_token` by reading the first two tokens.
    pub fn new(mut lexer: lexer::Lexer) -> Self {
        let current_token = lexer.next_token();

        let peek_token = lexer.next_token();

        let parser = Parser {
            lexer: lexer,
            current_token: current_token,
            peek_token,
        };

        
        parser
    }

    pub fn next_token(&mut self) {
        // When current token is EOF, peek token is next EOF.
        // Lexer.position point to the third EOF,
        // while Lexer.read_position points to the fourth EOF.
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Vec<ParseError>> { 
        let mut statements: Vec<Statement> = Vec::new();
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


    fn peek_precedence(&self) -> Precedence {
        if let Some(prec) = Precedence::from_token(self.peek_token.token_type) {
            prec
        } else {
            Precedence::Lowest
        }
    }

    fn current_precedence(&self) -> Precedence {
        if let Some(prec) = Precedence::from_token(self.current_token.token_type) {
            prec
        } else {
            Precedence::Lowest
        }
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
                expected: token_type, 
                got: self.peek_token.token_type,
            })
        }
    }

}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
pub use test_utils::*;