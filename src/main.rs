pub mod token;
pub mod lexer;
pub mod repl;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start();
}
