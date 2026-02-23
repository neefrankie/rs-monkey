use crate::ast::{self};
use crate::lexer;
use crate::token::{self, TokenType};
use std::vec::Vec;

pub struct Parser {
    lexer: lexer::Lexer,
    pub current_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
}

impl Parser {
    /// Creates a new parser.
    /// Initializes `current_token` and `peek_token` by reading the first two tokens.
    pub fn new(mut lexer: lexer::Lexer) -> Self {
        let current_token = lexer.next_token();
        println!("New Parser: current_token: {:?}", current_token);

        let peek_token = lexer.next_token();

        println!("New Parser: peek_token: {:?}", peek_token);

        let parser = Parser {
            lexer: lexer,
            current_token: current_token,
            peek_token,
            errors: Vec::new(),
        };

        
        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        println!("Parser: current_token: {:?}, peek_token: {:?}", self.current_token, self.peek_token)
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> { 
        let mut statements: Vec<Box<dyn ast::Statement>> = Vec::new();

        while self.current_token.token_type != TokenType::Eof {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                statements.push(stmt);
            }
            self.next_token();
        }
        return Some(ast::Program {
            statements,
        });
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> { 
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        // For let x = 5;, current_token points to `let`.
        // expect_peek might move the pointer, so we need to clone it.
        let let_token = self.current_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        // For let x = 5;, expect_peek moves current_token to x.
        let name = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }


        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(Box::new(ast::LetStatement {
            token: let_token,
            name,
            value: None,
        }));
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        // For return 5;, current_token points to `return`.
        let return_token = self.current_token.clone();

        self.next_token();
        println!("parse_return: move to next token {}", self.current_token.literal);

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
            println!("parse_return: skip next token {}", self.current_token.literal)
        }

        return Some(Box::new(ast::ReturnStatement {
            token: return_token,
            return_value: None,
        }))
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let error = format!("Expected next token to be {:?}, got {:?}", token_type, self.peek_token.token_type);
        self.errors.push(error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Node;
    use crate::lexer;

    fn as_let_statement(stmt: &dyn ast::Statement) -> Option<&ast::LetStatement> {
        stmt.as_any().downcast_ref::<ast::LetStatement>()
    }

    fn as_return_statement(stmt: &dyn ast::Statement) -> Option<&ast::ReturnStatement> {
        stmt.as_any().downcast_ref::<ast::ReturnStatement>()
    }

    fn test_let_statement(stmt: &dyn ast::Statement, name: &str) -> bool {
        if stmt.token_literal() != "let".to_string() {
            println!("Expected 'let', got '{}'", stmt.token_literal());
            return false;
        }

        let let_stmt = match as_let_statement(stmt) {
            Some(let_stmt) => let_stmt,
            None => {
                println!("Statement is not a LetStatement");
                return false;
            }
        };

        if let_stmt.name.value != name {
            println!("Expected '{}', got '{}'", name, let_stmt.name.value);
            return false;
        }

        if let_stmt.name.token.literal != name {
            println!("Expected '{}', got '{}'", name, let_stmt.name.token.literal);
            return false;
        }

        true
    }

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors.clone();
        if errors.len() == 0 {
            return;
        }

        println!("Parser has {} errors", errors.len());
        for error in errors {
            println!("Parser error: {}", error);
        }
        panic!("Parser has errors")

    }

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;
";
        let lex = lexer::Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let programm = parser.parse_program()
            .expect("parse_program failed");
        check_parser_errors(&parser);

        assert_eq!(programm.statements.len(), 3,
            "program.statements does not contain 3 statements. got={}",
            programm.statements.len());

        let tests = vec![
            "x",
            "y",
            "foobar",
        ];

        for (i, &expected_identifier) in tests.iter().enumerate() {
            // &* 先解引用得到 dyn Statement，再取引用得到 &dyn Statement
            let stmt = &*programm.statements[i];

            assert!(
                test_let_statement(stmt, expected_identifier),
                "Test failed for statement {} with identifier {}",
                i,
                expected_identifier
            )
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "return 5;
return 10;
return 993 322;
";
        let lex = lexer::Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let program = parser.parse_program()
            .expect("parse_program failed");
        
        check_parser_errors(&parser);
        
        assert_eq!(program.statements.len(), 3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len());

        for box_stmt in program.statements {
            let stmt = &*box_stmt;

            let ret_stmt = match as_return_statement(stmt) {
                Some(ret_stmt) => ret_stmt,
                None => {
                    println!("Statement is not a ReturnStatement");
                    continue;
                }
            };

            assert_eq!(
                ret_stmt.token_literal(),
                "return",
                "return statement does not 'return', got {}",
                ret_stmt.token_literal()
            )
        }
    }
}