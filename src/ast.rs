use crate::token;

mod statements;
mod expressions;
mod program;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug, Clone)]
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
    // x + 10;
    Expression {
        token: token::Token, // x
        expression: Box<Expression>, // x + 10
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
        // x > y
        token: token::Token, // >
        left: Box<Expression>, // x
        operator: String, // >
        right: Box<Expression>, // y
    },
    If {
        // if (x > y) {
        //     return x;
        // } else {
        //     return y;
        // }
        token: token::Token, // if
        condition: Box<Expression>, // x > y
        consequence: BlockStatement, // { return x; }
        alternative: Option<BlockStatement>, // { return y; }
    },
    FunctionLiteral {
        // fn(x, y) {
        //     return x + y;
        // }
        token: token::Token, // fn
        parameters: Vec<Identifier>, // [x, y]
        body: BlockStatement, // { return x + y; }
    },
    Call {
        // add(x, y)
        token: token::Token, // add
        function: Box<Expression>, // add
        arguments: Vec<Expression>, // [x, y]
    },
}


pub struct Program {
    pub statements: Vec<Statement>,
}



#[cfg(test)]
mod tests;