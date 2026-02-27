use crate::token;


mod statements;
mod expressions;
mod program;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String, // token.literal
}

#[derive(Debug)]
pub enum Statement {
    Let {
        token: token::Token,
        name: Identifier,
        value: Box<Expression>,
    },
    Return {
        token: token::Token,
        return_value: Option<Box<Expression>>,
    },
    Expression {
        token: token::Token,
        expression: Box<Expression>,
    },
}

#[derive(Debug)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Expression {
    Ident(Identifier),
    IntegerLiteral {
        token: token::Token,
        value: i64,
    },
    Boolean {
        token: token::Token,
        value: bool,
    },
    Prefix {
        token: token::Token,
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        token: token::Token,
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    If {
        token: token::Token,
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    FunctionLiteral {
        token: token::Token,
        parameters: Vec<Identifier>,
        body: BlockStatement,
    },
    Call {
        token: token::Token,
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}


pub struct Program {
    pub statements: Vec<Statement>,
}



#[cfg(test)]
mod tests;