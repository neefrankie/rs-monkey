use std::rc::Rc;
use crate::ast::{self, BlockStatement, Expression, Identifier};
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

            TokenType::Bang |
            TokenType::Minus => self.parse_prefix_expression(),

            TokenType::True |
            TokenType::False => self.parse_boolean(),

            TokenType::LParen => self.parse_grouped_expression(),

            TokenType::If => self.parse_if_expression(),

            TokenType::Function => self.parse_function_literal(),

            _ => Err(ParseError::NoPrefixParseFn {
                token_type
            })
        }
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParseError> {
        return Ok(Expression::Ident(ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }));
    }

    fn parse_integer(&mut self) -> Result<Expression, ParseError> {
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

    fn parse_boolean(&mut self) -> Result<Expression, ParseError> { 
        return Ok(Expression::Boolean {
            token: self.current_token.clone(),
            value: self.current_token_is(TokenType::True),
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
            right: Rc::new(right),
        });
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest)?;

        self.expect_peek(TokenType::RParen)?;

        return Ok(exp);
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        // if (x > y) {
        //     return x;
        // } else {
        //     return y;
        // }
        let if_token = self.current_token.clone();

        // Move to (.
        self.expect_peek(TokenType::LParen)?;
        // Move to x.
        self.next_token();
        // x > y
        let condition = self.parse_expression(Precedence::Lowest)?;
        // Move to )
        self.expect_peek(TokenType::RParen)?;
        // Move to {.
        self.expect_peek(TokenType::LBrace)?;

        let consequence = self.parse_block_statement()?;

        // Current token now points to }.
        let mut alternative: Option<Rc<BlockStatement>> = None;
        // Is next token an else?
        // 如果遇到 else，则将词法单元前移两位。
        if self.peek_token_is(TokenType::Else) {
            // 前移 Skip the else token
            self.next_token();
            // Next token should be an opening brace
            // 前移 Skip the opening brace
            self.expect_peek(TokenType::LBrace)?;

            let alt = self.parse_block_statement()?;

            alternative = Some(Rc::new(alt));
        }

        return Ok(Expression::If {
            token: if_token,
            condition: Rc::new(condition),
            consequence: Rc::new(consequence),
            alternative: alternative,
        });
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParseError> {
        // fn (x, y) {
        //  return x + y;
        // }
        // fn () {
        //  return foobar + barfoo;
        // }
        // let myFunction = fn (x, y) {
        //  return x + y;
        // };
        // fn() {
        //  return fn(x, y) { return x + y;};
        // }
        // myFunc(x, y, fn(x, y) { return x + y; });
        // 
        // Current token: fn
        let token = self.current_token.clone();
        // Current token: (
        self.expect_peek(TokenType::LParen)?;

        let parameters = self.parse_function_parameters()?;

        self.expect_peek(TokenType::LBrace)?;

        let body = self.parse_block_statement()?;

        Ok(Expression::FunctionLiteral {
            token,
            parameters,
            body: Rc::new(body),
        })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, ParseError> {
        // Current token: (
        let mut identifiers: Vec<Identifier> = vec![];
        
        if self.peek_token_is(TokenType::RParen) {
            // Skip ).
            self.next_token();
            return Ok(identifiers);
        }
        // Skip (.
        self.next_token();

        let ident = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };
        identifiers.push(ident);

        while self.peek_token_is(TokenType::Comma) {
            // Skip ,.
            self.next_token();
            // Point to next parameter.
            self.next_token();

            let ident = Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            };
            identifiers.push(ident);
        }
        // Expect ).
        self.expect_peek(TokenType::RParen)?;

        return Ok(identifiers);
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

            TokenType::LParen => self.parse_call_expression(left),

            _ => Err(ParseError::NoInfixParseFn {
                token_type
            })
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        println!("parse_infix_expression\n");
        // current token points to +, -, /, *, ==, !=, <, >.
        let current_token = self.current_token.clone();
        let operator = self.current_token.literal.clone();

        let precedence = self.current_precedence();
        self.next_token();

        println!("parse_infix_expression: start parsing right expression");

        let right = self.parse_expression(precedence)?;

        println!("parse_infix_expression: finish parsing right expression: {}", right);

        return Ok(Expression::Infix {
            token: current_token,
            left: Rc::new(left),
            operator,
            right: Rc::new(right),
        })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParseError> {
        // For function call, add(2, 3), the left
        // parenthesis is an infix operator.
        // current token points to '('
        let token = self.current_token.clone();
        let args = self.parse_call_arguments()?;
        
        return Ok(Expression::Call {
            token,
            function: Rc::new(function),
            arguments: args,
        })
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, ParseError> {
        // current token points to '('
        let mut args: Vec<Expression> = Vec::new();

        // Empty arguments.
        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Ok(args);
        }
        // Skip '('
        self.next_token();
        let arg = self.parse_expression(Precedence::Lowest)?;
        args.push(arg);
        
        while self.peek_token_is(TokenType::Comma) {
            // Skip ','
            self.next_token();
            // Check if next token is ) here?
            // Next argument.
            self.next_token();
            let arg = self.parse_expression(Precedence::Lowest)?;
            args.push(arg);
        }

        self.expect_peek(TokenType::RParen)?;

        Ok(args)
    }
}