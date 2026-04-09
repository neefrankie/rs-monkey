use std::{rc::Rc};

use crate::token;

mod statement;
mod expression;
mod program;

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
        token: token::Token, // Let
        name: Identifier, // 变量名
        value: Rc<Expression>, // 等号右侧的表达式
    },
    // return;
    // return 10;
    Return {
        token: token::Token,
        return_value: Option<Rc<Expression>>,
    },
    // x + 10;
    Expression {
        token: token::Token, // x
        expression: Rc<Expression>, // x + 10
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
    StringLiteral {
        token: token::Token,
        value: String,
    },
    Prefix {
        // -5;
        // !foobar;
        // 5 + -10;
        token: token::Token,
        operator: String, // ! or -
        right: Rc<Expression>, // Expression on the right of ! or -
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
        parameters: Vec<Identifier>, // (x, y)
        body: Rc<BlockStatement>, // { return x + y; }
    },
    Call {
        // add(x, y)
        // fn(x, y) { x + y; }(2, 3)
        token: token::Token, // （
        function: Rc<Expression>, // Identifier or function literal
        arguments: Vec<Expression>, // [x, y]
    },
    ArrayLiteral {
        token: token::Token, // [
        elements: Vec<Expression>,
    },
    Index {
        token: token::Token, // [
        left: Rc<Expression>,
        index: Rc<Expression>,
    },
    HashLiteral {
        token: token::Token, // {
        pairs: Vec<(Expression, Expression)>,
    },
}


#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String, // token.literal
}



#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
pub use test_utils::*;