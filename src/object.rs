use std::{fmt};

mod environment;

pub use environment::Environment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
    Error(String)
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::ReturnValue(obj) => obj.inspect(),
            Object::Null => "null".to_string(),
            Object::Error(message) => format!("ERROR: {}", message),
        }
    }

    pub fn type_of(&self) -> String {
        match self {
            Object::Integer(_) => "INTEGER".to_string(),
            Object::Boolean(_) => "BOOLEAN".to_string(),
            Object::ReturnValue(_) => "RETURN_VALUE".to_string(),
            Object::Null => "NULL".to_string(),
            Object::Error(_) => "ERROR_OBJ".to_string(),
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

    pub fn is_error(&self) -> bool {
        match self {
            Object::Error(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.type_of(), self.inspect())
    }
}