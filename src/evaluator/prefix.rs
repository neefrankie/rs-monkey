use crate::object::{Object};
use super::error::{EvalError};

pub(super) fn eval_prefix_expression(operator: String, right: Object) -> Result<Object, EvalError> {
    match operator.as_str() {
        "!" => Ok(eval_bang_operator_expression(right)),
        "-" => eval_minus_operator_expression(right),
        _ => Err(EvalError::UnknownPrefix {
            operator, 
            type_name: right.type_name().to_string(),
        })
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean(value) => {
            if value {
                Object::Boolean(false)
            } else {
                Object::Boolean(true)
            }
        }
        Object::Null => Object::Boolean(true),
        
        _ => Object::Boolean(false)

    }
}

fn eval_minus_operator_expression(right: Object) -> Result<Object, EvalError> {
    match right {
        Object::Integer(value) => {
            Ok(Object::Integer(-value))
        }
        _ => Err(EvalError::UnknownPrefix { 
            operator: "-".to_string(), 
            type_name: right.type_name().to_string()
        })
    }
}