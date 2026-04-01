use std::{fmt, error};

use crate::object::{TYPE_NAME_BOOLEAN, TYPE_NAME_INTEGER, TYPE_NAME_STRING};

#[derive(Debug)]
pub enum EvalError {
    UnknownIdentifier(String),
    UnknownOperator(String),
    UnknownPrefix{
        operator: String,
        type_name: String,
    },
    TypeMismatch{
        left_type: String,
        operator: String,
        right_type: String,
    },
    UnknownInfix{
        left_type: String,
        operator: String,
        right_type: String,
    },
    UnknownFunction(String),
}

pub fn new_unknown_boolean_infix(operator: String) -> EvalError {
    EvalError::UnknownInfix{
        left_type: TYPE_NAME_BOOLEAN.to_string(),
        operator,
        right_type: TYPE_NAME_BOOLEAN.to_string(),
    }
}

pub fn new_unknown_integer_infix(operator: String) -> EvalError {
    EvalError::UnknownInfix{
        left_type: TYPE_NAME_INTEGER.to_string(),
        operator,
        right_type: TYPE_NAME_INTEGER.to_string(),
    }
}

pub fn new_unknown_string_infix(operator: String) -> EvalError {
    EvalError::UnknownInfix{
        left_type: TYPE_NAME_STRING.to_string(),
        operator,
        right_type: TYPE_NAME_STRING.to_string(),
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::UnknownIdentifier(ident) => {
                write!(f, "identifier not found: {}", ident)
            }

            EvalError::UnknownOperator(operator) => {
                write!(f, "unknown operator: {}", operator)
            }

            EvalError::UnknownPrefix{operator, type_name} => {
                write!(f, "unknown operator: {}{}", operator, type_name)
            }

            EvalError::TypeMismatch{left_type, operator, right_type} => {
                write!(f, "type mismatch: {} {} {}", left_type, operator, right_type)
            }

            EvalError::UnknownInfix{left_type, operator, right_type} => {
                write!(f, "unknown operator: {} {} {}", left_type, operator, right_type)
            }

            EvalError::UnknownFunction(name) => {
                write!(f, "not a function: {}", name)
            }
        }
    }
}

impl error::Error for EvalError {}