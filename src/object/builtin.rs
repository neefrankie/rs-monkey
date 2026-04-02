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
        Object::String(s) => Ok(Object::Integer(s.len() as i64)),
        _ => Err(EvalError::TypeMismatch {
            expected: "string".to_string(),
            got: format!("{:?}", args[0]),
        })
    }
}

pub fn builtins() -> HashMap<String, BuiltinFunction> {
    let mut builtins: HashMap<String, BuiltinFunction> = HashMap::new();

    builtins.insert("len".to_string(), builtin_len);

    builtins
}