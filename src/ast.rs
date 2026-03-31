use crate::token;
use std::rc::Rc;

mod statements;
mod expressions;
mod program;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub struct Program {
    pub statements: Vec<Statement>,
}

// When Expression is used in a field, you have to wrap in it a Box
// since this is a recursive:
// BlockStatement -> Statement -> Expression -> BlockStatement -> ...
// For &Box<Expression>, yod don't need to manually deref it.
// It could be passed directly as &Expression.
#[derive(Debug)]
pub enum Statement {
    // let x = 5;
    Let {
        token: token::Token,
        name: Identifier,
        value: Box<Expression>,
    },
    // return;
    // return 10;
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
        right: Rc<Expression>,
    },
    Infix {
        // x > y
        token: token::Token, // >
        left: Rc<Expression>, // x
        operator: String, // >
        right: Rc<Expression>, // y
    },
    If {
        // if (x > y) {
        //     return x;
        // } else {
        //     return y;
        // }
        token: token::Token, // if
        condition: Rc<Expression>, // x > y
        consequence: Rc<BlockStatement>, // { return x; }
        alternative: Option<Rc<BlockStatement>>, // { return y; }
    },
    FunctionLiteral {
        // fn(x, y) {
        //     return x + y;
        // }
        token: token::Token, // fn
        parameters: Vec<Identifier>, // [x, y]
        body: Rc<BlockStatement>, // { return x + y; }
    },
    Call {
        // add(x, y)
        token: token::Token, // add
        function: Rc<Expression>, // add
        arguments: Vec<Expression>, // [x, y]
    },
}


#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String, // token.literal
}



#[cfg(test)]
mod tests;