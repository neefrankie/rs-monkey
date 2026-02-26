use crate::ast::{self, Expression};
use crate::token::{TokenType};

use super::errors::ParseError;
use super::precedence::{Precedence};
use super::Parser;

impl Parser {
    pub(super) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        println!("parse_expression({:?})\n", precedence);

        let mut left_expr = self.parse_prefix(self.current_token.token_type)?;

        println!("parse_expression left_expr: {}\n", left_expr);

        println!("parse_expression start nesting: precedence: {:?} < {:?}", precedence, self.peek_precedence());

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left_expr = self.parse_infix(
                self.current_token.token_type,
                left_expr
            )?;
        }
        
        println!("\tnest infix end");

        return Ok(left_expr);
    }

    fn parse_prefix(&mut self, token_type: TokenType) -> Result<Expression, ParseError> {
        print!("parse_prefix\n");
        match token_type {
            TokenType::Ident => self.parse_identifier(),
            TokenType::Int => self.parse_integer(),
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
            _ => Err(ParseError::NoPrefixParseFn {
                token_type
            })
        }
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParseError> {
        println!("parse_identifier: {}\n", self.current_token.literal);

        return Ok(Expression::Ident(ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }));
    }

    fn parse_integer(&mut self) -> Result<Expression, ParseError> {
        println!("parse_integer\n");

        let Ok(value) = self.current_token.literal.parse::<i64>() else {
            return Err(ParseError::ExpectedInt {
                got: self.current_token.literal.clone(),
            });
        };

        return Ok(Expression::IntegerLiteral {
            token: self.current_token.clone(),
            value: value,
        });
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        println!("parse_prefix_expression\n");
        let current_token = self.current_token.clone();
        let operator = self.current_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::Prefix)?;

        return Ok(Expression::Prefix {
            token: current_token,
            operator,
            right: Box::new(right),
        });
    }

    fn parse_infix(&mut self, token_type: TokenType, left: Expression) -> Result<Expression, ParseError> {
        print!("parse_infix {:?}\n", token_type);
        match token_type {
            TokenType::Plus |
            TokenType::Minus |
            TokenType::Slash |
            TokenType::Asterisk |
            TokenType::Eq |
            TokenType::NotEq |
            TokenType::LessThan |
            TokenType::GreaterThan => self.parse_infix_expression(left),

            _ => Err(ParseError::NoInfixParseFn {
                token_type
            })
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        println!("parse_infix_expression\n");

        let current_token = self.current_token.clone();
        let operator = self.current_token.literal.clone();

        let precedence = self.current_precedence();
        self.next_token();

        println!("parse_infix_expression: start parsing right expression");

        let right = self.parse_expression(precedence)?;

        println!("parse_infix_expression: finish parsing right expression: {}", right);

        return Ok(Expression::Infix {
            token: current_token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}