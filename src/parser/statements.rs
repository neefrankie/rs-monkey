use std::rc::Rc;
use crate::ast::{self, Statement, BlockStatement};
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

    pub(super) fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        
        print!("parse_let_statement start: {}\n", self.current_token.literal);

        // current_token points to `let`.
        let let_token = self.current_token.clone();
        // Move to identifier.
        self.expect_peek(TokenType::Ident)?;

        // Parse identifier.
        let name = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };
        // Next token must be '=' and move to it.
        self.expect_peek(TokenType::Assign)?;

        // Go to token after '='
        self.next_token(); 
        
        // Point to the start of expression.
        let value = self.parse_expression(Precedence::Lowest)?;

        // stops at ; token.
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        print!("parse_let_statement ends: {}\n", self.current_token.literal);

        return Ok(Statement::Let {
            token: let_token,
            name,
            value: Rc::new(value),
        });
    }

    pub(super) fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        print!("parse_return_statement starts: {}\n", self.current_token.literal);

        // current_token points to `return` in return 5;
        let return_token = self.current_token.clone();

        // Move to token after `return`
        self.next_token();

        // return value should be None if there is no expression after `return`
        // return;
        // return 5;
        // return 5 + 5;
        // 如果遇到分号、EOF 或右花括号（函数体结束），说明没有返回值
        let return_value = if self.current_token_is(TokenType::Semicolon)
            || self.current_token_is(TokenType::Eof)
            || self.current_token_is(TokenType::RightBrace) {
                None
            } else {
                // For return 993 322;,
                // only 993 is parsed.
                // When it comes to 322,
                // precedence(0) == peek_precedence(0),
                // the parser has no idea what to do with it.
                // The following while loop will simply drop it.
                let expr = self.parse_expression(Precedence::Lowest)?;
                Some(Rc::new(expr))
            };
        
        // Stops at ;
        // TODO: This differs from the book's implementation
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        print!(
            "parse_return_statement ends: {}\n",
            self.current_token.literal
        );

        return Ok(Statement::Return { 
            token: return_token, 
            return_value: return_value, 
        })
    }

    pub(super) fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        
        print!("parse_expression_statement start: {}\n", self.current_token.literal);

        // current_token points to `x` in x + 10;
        let current_token = self.current_token.clone();
        // parsing x + 10
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Move to ; if exists
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        print!("parse_expression_statement end: {}\n", self.current_token.literal);

        return Ok(Statement::Expression {
            token: current_token,
            expression: Rc::new(expr),
        });
    }

    pub(super) fn parse_block_statement(&mut self) -> Result<BlockStatement, ParseError> {
        // curent token point to {.
        let token = self.current_token.clone();
        let mut statements: Vec<Statement> = Vec::new();

        self.next_token();
        // 反复调用 parse_statement，直到遇见右大括号 }
        // 或 TokenType::Eof，前者表示到了块语句的末尾，
        // 后者表示没有要解析的词法单元。
        // Stops at } or EOF
        while !self.current_token_is(TokenType::RightBrace) && !self.current_token_is(TokenType::Eof) {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token();
        }

        return Ok(BlockStatement {
            token,
            statements,
        });
    }
}