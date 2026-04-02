use crate::object::{Object, EvalError};

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
        (Object::String(left_value), Object::String(right_value)) => {
            eval_string_infix_expression(
                operator, 
                left_value, 
                right_value
            )
        }
        _ => {            
            return Err(EvalError::UnknownInfix { 
                left: left.type_name().to_string(), 
                operator, 
                right: right.type_name().to_string(), 
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
        _ => Err(EvalError::UnknownInfix {
            left: left.to_string(),
            operator,
            right: right.to_string(),
        })
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
        _ => Err(EvalError::UnknownInfix {
            left: left.to_string(), 
            operator, 
            right: right.to_string(), 
        }),
    }
}

fn eval_string_infix_expression(
    operator: String, 
    left: &str, 
    right: &str
) -> Result<Object, EvalError> {
    match operator.as_str() {
        "+" => Ok(Object::String(format!("{}{}", left, right))),
        _ => Err(EvalError::UnknownInfix {
            left: left.to_string(),
            operator,
            right: right.to_string(),
        }),
    }
}