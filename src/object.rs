use std::{cell::RefCell, fmt, rc::Rc};
use crate::{ast};

mod environment;
mod builtin;
mod error;

pub use environment::Environment;
pub use error::EvalError;

pub type BuiltinFunction = fn(Vec<Object>) -> Result<Object, EvalError>;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
    Error(String),
    Function {
        parameters: Vec<ast::Identifier>,
        body: Rc<ast::BlockStatement>,
        env: Rc<RefCell<Environment>>,
    },
    String(String),
    Builtin(BuiltinFunction)
}

impl Object {

    pub fn type_name(&self) -> &'static str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Null => "NULL",
            Object::Error(_) => "ERROR",
            Object::Function { .. } => "FUNCTION",
            Object::String(_) => "STRING",
            Object::Builtin(_) => "BUILTIN_OBJ",
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Object::Null => true,
            _ => false,
        }
    }

    pub fn is_return(&self) -> bool {
        match self {
            Object::ReturnValue(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            Object::Null => write!(f, "null"),
            Object::Error(message) => write!(f, "ERROR: {}", message),
            Object::Function { 
                parameters, 
                body, 
                ..
            } => {
                let params = parameters.iter()
                    .map(
                        |p| p.to_string()
                    )
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(
                    f,
                    "fn ({}) {{\n{}\n}}",
                    params, body
                )
            },
            Object::String(value) => write!(f, "{}", value),
            Object::Builtin(_) => write!(f, "builtin function"),
        }
    }
}
