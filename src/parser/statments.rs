use crate::ast::{self, Statement};
use crate::token::{TokenType};

use super::errors::ParseError;
use super::precedence::{Precedence};
use super::Parser;

impl Parser {
    pub(super) fn parse_statement(&mut self) -> Result<Statement, ParseError> { 
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        // For let x = 5;, current_token points to `let`.
        // expect_peek might move the pointer, so we need to clone it.
        let let_token = self.current_token.clone();

        self.expect_peek(TokenType::Ident)?;

        // Parse x in x = 5
        let name = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        self.expect_peek(TokenType::Assign)?;

        self.next_token(); // consume '='
        
        // Current token points to '5' in let x = 5;
        let value = self.parse_expression(Precedence::Lowest)?;

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Ok(Statement::Let {
            token: let_token,
            name,
            value: Box::new(value),
        });
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        // current_token points to `return` in return 5;
        let return_token = self.current_token.clone();

        // Move to token after `return`
        self.next_token();

        println!("parse_return: move to next token {}", self.current_token.literal);

        // return value should be None if there is no expression after `return`
        // return;
        // return 5;
        // return 5 + 5;
        // 如果遇到分号、EOF 或右花括号（函数体结束），说明没有返回值
        let return_value = if self.current_token_is(TokenType::Semicolon)
            || self.current_token_is(TokenType::Eof)
            || self.current_token_is(TokenType::RBrace) {
                None
            } else {
                let expr = self.parse_expression(Precedence::Lowest)?;
                Some(Box::new(expr))
            };

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();

            println!("parse_return: skip next token {}", self.current_token.literal)
        }

        return Ok(Statement::Return { 
            token: return_token, 
            return_value: return_value, 
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        // x + 10;
        let current_token = self.current_token.clone();
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Ok(Statement::Expression {
            token: current_token,
            expression: Box::new(expr),
        });
    }
}