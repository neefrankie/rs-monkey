use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use crate::{ast};

mod environment;
mod builtin;
mod error;

pub use environment::Environment;
pub use error::EvalError;

pub type BuiltinFunction = fn(Vec<Object>) -> Result<Object, EvalError>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HashKey {
    Integer(i64),
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone)]
pub struct HashPair {
    pub key: Object,
    pub value: Object,
}

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
    Builtin(BuiltinFunction),
    Array(Vec<Object>),
    Hash(HashMap<HashKey, HashPair>),
}

impl Object {

    pub fn type_name(&self) -> String {
        match self {
            Object::Integer(_) => "INTEGER".to_string(),
            Object::Boolean(_) => "BOOLEAN".to_string(),
            Object::ReturnValue(_) => "RETURN_VALUE".to_string(),
            Object::Null => "NULL".to_string(),
            Object::Error(_) => "ERROR".to_string(),
            Object::Function { .. } => "FUNCTION".to_string(),
            Object::String(_) => "STRING".to_string(),
            Object::Builtin(_) => "BUILTIN_OBJ".to_string(),
            Object::Array(_) => "ARRAY".to_string(),
            Object::Hash(_) => "HASH".to_string(),
        }
    }

    pub fn hash_key(&self) -> Result<HashKey, EvalError> {
        match self {
            Object::Boolean(value) => {
                Ok(HashKey::Boolean(*value))
            },
            Object::Integer(value) => {
                Ok(HashKey::Integer(*value))
            },
            Object::String(value) => {
                Ok(HashKey::String(value.clone()))
            },
            _ => Err(EvalError::InvalidHashKey(
                self.type_name()
            )),
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
            Object::Array(elements) => {
                let elements = elements.iter()
                    .map(
                        |e| e.to_string()
                    )
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(
                    f,
                    "[{}]",
                    elements
                )
            },
            Object::Hash(hash) => {
                let pairs = hash.values()
                    .map(
                        |pair| format!("{}: {}", pair.key, pair.value)
                    )
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(
                    f,
                    "{{{}}}",
                    pairs
                )
            },
        }
    }
}
