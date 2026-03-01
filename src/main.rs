pub mod token;
pub mod lexer;
pub mod repl;
pub mod ast;
pub mod parser;
pub mod object;
pub mod evaluator;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start();
}
