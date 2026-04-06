use std::rc::Rc;
use crate::ast::{self, BlockStatement, Expression, Identifier};
use crate::token::{TokenType};

use super::errors::ParseError;
use super::precedence::{Precedence};
use super::Parser;

impl Parser {
    pub(super) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let mut left_expr = self.dispatch_prefix_parsing(self.current_token.token_type)?;

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left_expr = self.parse_infix(
                self.current_token.token_type,
                left_expr
            )?;
        }

        return Ok(left_expr);
    }

    // This function plays the role of registerPrefix(TokenType, func) in Go version.
    fn dispatch_prefix_parsing(&mut self, token_type: TokenType) -> Result<Expression, ParseError> {
        match token_type {
            TokenType::Ident => Ok(self.parse_identifier()),

            TokenType::Int => self.parse_integer(),

            TokenType::True |
            TokenType::False => Ok(self.parse_boolean()),

            TokenType::String => Ok(self.parse_string_literal()),

            TokenType::Bang |
            TokenType::Minus => self.parse_prefix_expression(),

            TokenType::LeftParen => self.parse_grouped_expression(),

            TokenType::If => self.parse_if_expression(),

            TokenType::Function => self.parse_function_literal(),

            TokenType::LeftBracket => self.parse_array_literal(),

            TokenType::LeftBrace => self.parse_hash_literal(),

            _ => Err(ParseError::NoPrefixParseFn {
                token_type
            })
        }
    }

    // === Simple prefix parsing ===

    pub fn parse_identifier(&mut self) -> Expression {
        return Expression::Ident(ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        });
    }

    pub fn parse_integer(&mut self) -> Result<Expression, ParseError> {
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

    pub fn parse_boolean(&mut self) -> Expression { 
        return Expression::Boolean {
            token: self.current_token.clone(),
            value: self.current_token_is(TokenType::True),
        };
    }

    pub fn parse_string_literal(&mut self) -> Expression {
        return Expression::StringLiteral {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };
    }

    // === Complex prefix parsing ===

    // 两种前缀运算符：! 和 -
    // -5;
    // !foobar;
    // 5 + -10;
    // <prefix><expression>;
    pub fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
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

    // 用括号给表达式分组以修改起优先级，从而影响上下文中求值的顺序
    // (5 + 5) * 2;
    pub fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest)?;

        // Stops at the right paren.
        self.expect_peek(TokenType::RightParen)?;

        return Ok(exp);
    }

    // if (<condition>)<result> else <alternative>
    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        // if (x > y) {
        //     return x;
        // } else {
        //     return y;
        // }
        let if_token = self.current_token.clone();

        // Move to (.
        self.expect_peek(TokenType::LeftParen)?;
        // Move to x.
        self.next_token();
        // x > y
        let condition = self.parse_expression(Precedence::Lowest)?;
        // Move to )
        self.expect_peek(TokenType::RightParen)?;
        // Move to {.
        self.expect_peek(TokenType::LeftBrace)?;

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
            self.expect_peek(TokenType::LeftBrace)?;

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
        self.expect_peek(TokenType::LeftParen)?;

        let parameters = self.parse_function_parameters()?;

        self.expect_peek(TokenType::LeftBrace)?;

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
        
        if self.peek_token_is(TokenType::RightParen) {
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
        self.expect_peek(TokenType::RightParen)?;

        return Ok(identifiers);
    }

    fn parse_array_literal(&mut self) -> Result<Expression, ParseError> {
        return Ok(Expression::ArrayLiteral {
            token: self.current_token.clone(),
            elements: self.parse_expression_list(TokenType::RightBracket)?,
        });
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Result<Vec<Expression>, ParseError> {
        let mut list: Vec<Expression> = Vec::new();
        if self.peek_token_is(end) {
            self.next_token();
            return Ok(list);
        }

        self.next_token();
        let element = self.parse_expression(Precedence::Lowest)?;
        list.push(element);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            let element = self.parse_expression(Precedence::Lowest)?;
            list.push(element);

        }

        self.expect_peek(end)?;

        return Ok(list);
    }

    fn parse_hash_literal(&mut self) -> Result<Expression, ParseError> {
        let token = self.current_token.clone();
        let mut pairs: Vec<(Expression, Expression)> = Vec::new();

        while !self.peek_token_is(TokenType::RightBrace) {
            // Skip {.
            self.next_token();
            // Stops before :
            let key = self.parse_expression(Precedence::Lowest)?;
            
            // Move to :
            self.expect_peek(TokenType::Colon)?;
            // Skip :
            self.next_token();
            // Parse value
            let value = self.parse_expression(Precedence::Lowest)?;
            pairs.push((key, value));

            if self.peek_token_is(TokenType::RightBrace) {
                continue;
            }

            self.expect_peek(TokenType::Comma)?;
        }

        self.expect_peek(TokenType::RightBrace)?;

        return Ok(Expression::HashLiteral {
            token: token,
            pairs: pairs,
        });
    }

    

    

    // === registerInfix ===

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
            // Function call
            TokenType::LeftParen => self.parse_call_expression(left),
            // Array or map index
            TokenType::LeftBracket => self.parse_index_expression(left),

            _ => Err(ParseError::NoInfixParseFn {
                token_type
            })
        }
    }

    // <expression><infix operator><expression>
    // This function actually starts from the infix operator.
    pub fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        // current token points to +, -, /, *, ==, !=, <, >.
        let current_token = self.current_token.clone();
        let operator = self.current_token.literal.clone();

        let precedence = self.current_precedence();
        self.next_token();

        let right = self.parse_expression(precedence)?;

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
        let args = self.parse_expression_list(TokenType::RightParen)?;
        
        return Ok(Expression::Call {
            token,
            function: Rc::new(function),
            arguments: args,
        })
    }

    // fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, ParseError> {
    //     // current token points to '('
    //     let mut args: Vec<Expression> = Vec::new();

    //     // Empty arguments.
    //     if self.peek_token_is(TokenType::RParen) {
    //         self.next_token();
    //         return Ok(args);
    //     }
    //     // Skip '('
    //     self.next_token();
    //     let arg = self.parse_expression(Precedence::Lowest)?;
    //     args.push(arg);
        
    //     while self.peek_token_is(TokenType::Comma) {
    //         // Skip ','
    //         self.next_token();
    //         // Check if next token is ) here?
    //         // Next argument.
    //         self.next_token();
    //         let arg = self.parse_expression(Precedence::Lowest)?;
    //         args.push(arg);
    //     }

    //     self.expect_peek(TokenType::RParen)?;

    //     Ok(args)
    // }

    

    fn parse_index_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let token = self.current_token.clone();
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenType::RightBracket)?;

        return Ok(Expression::Index {
            token,
            left: Rc::new(left),
            index: Rc::new(index),
        });
    }
}