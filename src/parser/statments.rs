use crate::ast;
use crate::token::{TokenType};

use super::errors::ParseError;
use super::precedence::{LOWEST};
use super::Parser;

impl Parser {
    pub(super) fn parse_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> { 
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        // For let x = 5;, current_token points to `let`.
        // expect_peek might move the pointer, so we need to clone it.
        let let_token = self.current_token.clone();

        self.expect_peek(TokenType::Ident)?;

        // For let x = 5;, expect_peek moves current_token to x.
        let name = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        self.expect_peek(TokenType::Assign)?;

        // self.next_token(); // consume '='
        
        // // 必须能解析出表达式
        // let value = self.parse_expression(Precedence::Lowest)
        //     .ok_or(ParseError::MissingExpression)?;
        // 临时：用占位符代替未实现的表达式解析
        let value = Box::new(ast::MissingExpression {
            token: self.current_token.clone(), // 或 Token::new(TokenType::Illegal, "<missing>")
        });

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Ok(Box::new(ast::LetStatement {
            token: let_token,
            name,
            value,
        }));
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        // For return 5;, current_token points to `return`.
        let return_token = self.current_token.clone();

        self.next_token();
        println!("parse_return: move to next token {}", self.current_token.literal);

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
            println!("parse_return: skip next token {}", self.current_token.literal)
        }

        return Ok(Box::new(ast::ReturnStatement {
            token: return_token,
            return_value: None,
        }))
    }

    fn parse_expression_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let current_token = self.current_token.clone();
        let expr = self.parse_expression(LOWEST)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Ok(Box::new(ast::ExpressionStatement {
            token: current_token,
            expression: expr,
        }));
    }
}