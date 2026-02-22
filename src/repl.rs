use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::token::{TokenType};
use crate::parser::Parser;

const PROMPT: &str = ">> ";

pub fn start() {
    let mut input = String::new();

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().expect("Faield to flush stdout");

        input.clear();

        if io::stdin().read_line(&mut input).is_err() {
            println!();
            break;
        }

        // 移除换行符（read_line 会保留 \n 或 \r\n）
        let line = input.trim_end();

        let lexer = Lexer::new(line.to_string());

        let mut parser = Parser::new(lexer);
        loop {
            
            parser.next_token();
            println!("Step a token");
            if parser.current_token.token_type == TokenType::Eof {
                break;
            }
        }
    }
}