use std::collections::HashMap;

use super::{BuiltinFunction, Object, EvalError};

fn builtin_len(args: Vec<Object>) -> Result<Object, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::WrongArgumentCount { 
            expected: 1, 
            got: args.len()
        })
    }

    match &args[0] {
        Object::String(s) => {
            Ok(Object::Integer(s.len() as i64))
        },
        Object::Array(elements) => {
            Ok(Object::Integer(elements.len() as i64))
        },
        _ => Err(EvalError::TypeMismatch {
            expected: "string".to_string(),
            got: format!("{:?}", args[0]),
        })
    }
}

fn builtin_first(args: Vec<Object>) -> Result<Object, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::WrongArgumentCount { 
            expected: 1, 
            got: args.len()
        })
    }

    match &args[0] {
        Object::Array(elements) => {
            if elements.is_empty() {
                return Ok(Object::Null)
            }

            return Ok(elements[0].clone())
        }
        _ => Err(EvalError::TypeMismatch {
            expected: "array".to_string(),
            got: format!("{:?}", args[0]),
        })
    }
}

fn builtin_last(args: Vec<Object>) -> Result<Object, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::WrongArgumentCount { 
            expected: 1, 
            got: args.len()
        })
    }

    match &args[0] {
        Object::Array(elements) => {
            if elements.is_empty() {
                return Ok(Object::Null)
            }

            return Ok(elements[elements.len() - 1].clone())
        }
        _ => Err(EvalError::TypeMismatch {
            expected: "array".to_string(),
            got: format!("{:?}", args[0]),
        })
    }
}

fn builtin_rest(args: Vec<Object>) -> Result<Object, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::WrongArgumentCount { 
            expected: 1, 
            got: args.len()
        })
    }

    match &args[0] {
        Object::Array(elements) => {
            if elements.is_empty() {
                return Ok(Object::Null)
            }

            return Ok(Object::Array(elements[1..].to_vec()))
        }

        _ => Err(EvalError::TypeMismatch {
            expected: "array".to_string(),
            got: format!("{:?}", args[0]),
        })
    }
}

fn builtin_push(args: Vec<Object>) -> Result<Object, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::WrongArgumentCount { 
            expected: 2, 
            got: args.len()
        })
    }

    match &args[0] {
        Object::Array(elements) => {
            let mut new_elements = elements.clone();
            new_elements.push(args[1].clone());

            return Ok(Object::Array(new_elements))
        }
        _ => Err(EvalError::TypeMismatch {
            expected: "array".to_string(),
            got: format!("{:?}", args[0]),
        })
    }
}

pub fn builtins() -> HashMap<String, BuiltinFunction> {
    let mut builtins: HashMap<String, BuiltinFunction> = HashMap::new();

    builtins.insert("len".to_string(), builtin_len);
    builtins.insert("first".to_string(), builtin_first);
    builtins.insert("last".to_string(), builtin_last);
    builtins.insert("rest".to_string(), builtin_rest);
    builtins.insert("push".to_string(), builtin_push);

    builtins
}