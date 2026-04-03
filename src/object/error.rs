use std::{fmt, error};

#[derive(Debug)]
pub enum EvalError {
    UnknownIdentifier(String),
    UnknownOperator(String),
    UnknownPrefix{
        operator: String,
        type_name: String,
    },
    UnknownInfix{
        left: String,
        operator: String,
        right: String,
    },
    UnknownFunction(String),
    WrongArgumentCount{
        expected: usize,
        got: usize,
    },
    TypeMismatch{
        expected: String,
        got: String,
    },
    IndexOutOfBounds{
        index: i64,
        max_index: i64,
    },
    InvalidHashKey(String),
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
                write!(f, "error prefix operator: {}{}", operator, type_name)
            }

            EvalError::UnknownInfix{left: left_type, operator, right: right_type} => {
                write!(f, "error infix operator: {} {} {}", left_type, operator, right_type)
            }

            EvalError::UnknownFunction(name) => {
                write!(f, "not a function: {}", name)
            }

            EvalError::WrongArgumentCount { expected, got } => {
                write!(f, "wrong argument count: expected {}, got {}", expected, got)
            }

            EvalError::TypeMismatch { expected, got } => {
                write!(f, "type mismatch: expected {}, got {}", expected, got)
            }

            EvalError::IndexOutOfBounds { index, max_index } => {
                write!(f, "index out of bounds: index {}, max index {}", index, max_index)
            },

            EvalError::InvalidHashKey(key) => {
                write!(f, "unusable as hash key: {}", key)
            }
        }
    }
}

impl error::Error for EvalError {}