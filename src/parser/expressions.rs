use std::rc::Rc;
use crate::ast::{self, BlockStatement, Expression, Identifier};
use crate::token::{TokenType};

use super::errors::ParseError;
use super::precedence::{Precedence};
use super::Parser;

impl Parser {
    pub(super) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {

        println!(
            "parse_expression({:?}), current: {:?}, peek: {:?}\n",
            precedence, 
            self.current_token, 
            self.peek_token
        );

        let mut left_expr = self.dispatch_prefix_parsing(self.current_token.token_type)?;

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left_expr = self.dispatch_infix(
                self.current_token.token_type,
                left_expr
            )?;
        }

        return Ok(left_expr);
    }

    // This function plays the role of registerPrefix(TokenType, func) in Go version.
    fn dispatch_prefix_parsing(&mut self, token_type: TokenType) -> Result<Expression, ParseError> {

        print!("dispatch_prefix_parsing: {}\n", token_type);

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

    pub(super) fn parse_identifier(&mut self) -> Expression {
        
        print!("parse_identifier {}\n", self.current_token.literal);
        
        return Expression::Ident(ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        });
    }

    pub(super) fn parse_integer(&mut self) -> Result<Expression, ParseError> {
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

    pub(super) fn parse_boolean(&mut self) -> Expression { 
        return Expression::Boolean {
            token: self.current_token.clone(),
            value: self.current_token_is(TokenType::True),
        };
    }

    pub(super) fn parse_string_literal(&mut self) -> Expression {
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
    pub(super) fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
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
    pub(super) fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest)?;

        // Stops at the right paren.
        self.expect_peek(TokenType::RightParen)?;

        return Ok(exp);
    }

    // if (<condition>)<result> else <alternative>
    pub(super) fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
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
        if self.peek_token_is(TokenType::Else) {
            // Skip the else token
            self.next_token();
            // Next token should be an opening brace
            // Skip the {
            self.expect_peek(TokenType::LeftBrace)?;

            let alt = self.parse_block_statement()?;

            alternative = Some(Rc::new(alt));
        }
        // Stops at final }
        return Ok(Expression::If {
            token: if_token,
            condition: Rc::new(condition),
            consequence: Rc::new(consequence),
            alternative: alternative,
        });
    }

    // fn <parameters> <block-statement>
    pub(super) fn parse_function_literal(&mut self) -> Result<Expression, ParseError> {
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
        // Move to (
        self.expect_peek(TokenType::LeftParen)?;

        // Parse parameters: (x, y).
        // Stops at )
        let parameters = self.parse_function_parameters()?;

        // Move to opening brace
        self.expect_peek(TokenType::LeftBrace)?;

        // Parse statement unitl closing brace
        let body = self.parse_block_statement()?;

        // Now stops at }
        Ok(Expression::FunctionLiteral {
            token,
            parameters,
            body: Rc::new(body),
        })
    }

    // Parse function parameters
    // (<parameter1>, <parameter2>, <parameter3>, ...)
    pub(super) fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, ParseError> {
        // Current token: (
        let mut identifiers: Vec<Identifier> = vec![];
        
        if self.peek_token_is(TokenType::RightParen) {
            // Empty paramter.
            // Move to ).
            self.next_token();
            return Ok(identifiers);
        }
        // Move to first parameter.
        self.next_token();

        let ident = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };
        identifiers.push(ident);

        // Loop as long as next token is ,
        while self.peek_token_is(TokenType::Comma) {
            // Move to ,
            self.next_token();
            // Move to next parameter.
            self.next_token();

            let ident = Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            };
            identifiers.push(ident);
        }
        // Stops at )
        self.expect_peek(TokenType::RightParen)?;

        return Ok(identifiers);
    }

    pub(super) fn parse_array_literal(&mut self) -> Result<Expression, ParseError> {
        print!("parse_array_literal {}\n", self.current_token.literal);

        return Ok(Expression::ArrayLiteral {
            token: self.current_token.clone(), // Left bracket [
            elements: self.parse_expression_list(TokenType::RightBracket)?,
        });
    }

    // {<expression>: {expression}, ...}
    pub(super) fn parse_hash_literal(&mut self) -> Result<Expression, ParseError> {
        // Starting {
        let token = self.current_token.clone();
        let mut pairs: Vec<(Expression, Expression)> = Vec::new();

        while !self.peek_token_is(TokenType::RightBrace) {
            // Skip {.
            self.next_token();
            // Parse key.
            let key = self.parse_expression(Precedence::Lowest)?;
            
            // Move to :
            self.expect_peek(TokenType::Colon)?;
            // Skip :
            self.next_token();
            // Parse value
            let value = self.parse_expression(Precedence::Lowest)?;
            pairs.push((key, value));

            // Next token could be an optional comma.
            if self.peek_token_is(TokenType::RightBrace) {
                break;
            }

            self.expect_peek(TokenType::Comma)?;
        }

        // Move to }
        self.expect_peek(TokenType::RightBrace)?;

        return Ok(Expression::HashLiteral {
            token: token,
            pairs: pairs,
        });
    }

    

    

    // === registerInfix ===

    fn dispatch_infix(&mut self, token_type: TokenType, left: Expression) -> Result<Expression, ParseError> {

        print!("dispatch_infix {}\n", token_type);

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
            // [] 紧跟在另一个表达式后面时才是中缀，否则就是前缀。
            TokenType::LeftBracket => self.parse_index_expression(left),

            _ => Err(ParseError::NoInfixParseFn {
                token_type
            })
        }
    }

    // <expression><infix operator><expression>
    // This function actually starts from the infix operator.
    pub(super) fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
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

    // Parse function call.
    // <expression>(<comma separated expressions>)
    pub(super) fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParseError> {
        // For function call like add(2, 3)
        // the left parenthesis is an infix operator.
        // current token points to '('
        let token = self.current_token.clone();
        let args = self.parse_expression_list(TokenType::RightParen)?;
        
        return Ok(Expression::Call {
            token,
            function: Rc::new(function),
            arguments: args,
        })
    }

    // Parse function call arguments, or array elements.
    // (arg1, arg2, ...)
    // [elem1, elem2, ...]
    pub(super) fn parse_expression_list(&mut self, end: TokenType) -> Result<Vec<Expression>, ParseError> {
        print!("parse_expression_list: {}", self.current_token.literal);

        // We are pointing to ( or [ now.
        let mut list: Vec<Expression> = Vec::new();
        // Empty list.
        if self.peek_token_is(end) {
            // Move to closing ) or ]
            self.next_token();
            return Ok(list);
        }

        // Move to first element
        self.next_token();
        // First element
        let element = self.parse_expression(Precedence::Lowest)?;
        list.push(element);

        while self.peek_token_is(TokenType::Comma) {
            // Skip comma
            self.next_token();
            // Next element
            self.next_token();
            let element = self.parse_expression(Precedence::Lowest)?;
            list.push(element);

        }
        // Move to end token ) or ]
        self.expect_peek(end)?;

        return Ok(list);
    }    

    // <expression>[<expression>]
    // 对于数组，如果没有设置 TokenType::LeftBracket => Some(Precedence::Index)，
    // 是可以正常解析的（按照原书的代码顺序），但是无法识别出来时中缀表达式，
    // 以 myArray[1] 为例，首先解析成 Identifier，但是因为没有 `[`
    // 对应的权重，所以 myArray 解析成 Identifier 就结束了，
    // 不会进入优先级的循环，这成了一个独立的 Statement。
    // 接下来走到 [ 时，就会当作数组字面量了。
    pub(super) fn parse_index_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        
        print!("parse_index_expression {}\n", self.current_token.literal);

        // Points to starting `[`
        let token = self.current_token.clone();
        // Expresions inside []
        self.next_token();

        let index = self.parse_expression(Precedence::Lowest)?;

        // Expect next token to be `]`
        self.expect_peek(TokenType::RightBracket)?;

        return Ok(Expression::Index {
            token,
            left: Rc::new(left),
            index: Rc::new(index),
        });
    }
}