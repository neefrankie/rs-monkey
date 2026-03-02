use crate::{object::Object};
use super::error::{EvalError, new_unknown_boolean_infix, new_unknown_integer_infix};

pub(super) fn eval_infix_expression(
    operator: String, 
    left: Object, 
    right: Object
) -> Result<Object, EvalError> {
    match (&left, &right) {
        (Object::Integer(left_value), Object::Integer(right_value)) => {
            eval_integer_infix_expression(
                operator, 
                *left_value, 
                *right_value
            )
        }
        (Object::Boolean(left_value), Object::Boolean(right_value)) => {
            eval_boolean_infix_expression(
                operator, 
                *left_value, 
                *right_value
            )
        }
        _ => {
            if &left.type_name() != &right.type_name() {
                return Err(EvalError::TypeMismatch { 
                    left_type: left.type_name().to_string(), 
                    operator, 
                    right_type: right.type_name().to_string(),
                });
            }
            
            return Err(EvalError::UnknownInfix { 
                left_type: left.type_name().to_string(), 
                operator, 
                right_type: right.type_name().to_string(), 
            });
        }
    }

}

fn eval_integer_infix_expression(operator: String, left: i64, right: i64) -> Result<Object, EvalError> {
    match operator.as_str() {
        "+" => Ok(Object::Integer(left + right)),
        "-" => Ok(Object::Integer(left - right)),
        "*" => Ok(Object::Integer(left * right)),
        "/" => Ok(Object::Integer(left / right)),
        ">" => Ok(Object::Boolean(left > right)),
        "<" => Ok(Object::Boolean(left < right)),
        "==" => Ok(Object::Boolean(left == right)),
        "!=" => Ok(Object::Boolean(left != right)),
        _ => Err(new_unknown_integer_infix(operator))
    }
}

fn eval_boolean_infix_expression(
    operator: String, 
    left: bool, 
    right: bool
) -> Result<Object, EvalError> {
    match operator.as_str() {
        "==" => Ok(Object::Boolean(left == right)),
        "!=" => Ok(Object::Boolean(left != right)),
        _ => Err(new_unknown_boolean_infix(operator)),
    }
}