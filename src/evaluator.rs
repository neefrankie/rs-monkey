use crate::ast::{self, Statement, Expression};
use crate::object::{Object, Environment};

pub fn eval_program(
    program: ast::Program,
    env: &mut Environment,
) -> Object {
    
    let mut result = Object::Null;

    for stmt in program.statements {
        result = eval_stmt(stmt, env);

        if let Object::ReturnValue(value) = result {
            return *value;
        } else if result.is_error() {
            return result;
        }
    }

    return result;
}

pub fn eval_block_statement(
    block: ast::BlockStatement,
    env: &mut Environment,
) -> Object {
    let mut result = Object::Null;

    for stmt in block.statements {
        result = eval_stmt(stmt, env);

        if !result.is_null() {
            if result.is_return() || result.is_error() {
                return result;
            }
        }
    }

    return result;
}


pub fn eval_stmt(stmt: Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::Expression {
            expression,
            ..
        } => {
            eval_expression(
                *expression,
                env,
            )
        }

        Statement::Return { 
            return_value,
            ..
        } => {
            match return_value {
                Some(value) => {
                    let val = eval_expression(
                        *value,
                        env,
                    );
                    if val.is_error() {
                        return val;
                    }
                    Object::ReturnValue(Box::new(val))
                },
                None => Object::ReturnValue(Box::new(Object::Null))
            }
        }

        Statement::Let {
            name,
            value,
            ..
        } => {
            // For let a = 5;, value is 5
            // For let a = b + c;, value is b + c
            // which should have been saved to env.
            let val = eval_expression(
                *value,
                env,
            );
            if val.is_error() {
                return val;
            }
            // Save a: 5
            env.set(name.value, val.clone());

            val
        }
    }
}

pub fn eval_expression(
    expression: Expression,
    env: &mut Environment,
) -> Object {
    match expression {
        Expression::Ident(identifier) => {
            eval_identifier(identifier, env)
        }
        Expression::IntegerLiteral { 
            value ,
            ..
        } => {
            Object::Integer(value)
        }

        Expression::Boolean { 
            value ,
            ..
        } => Object::Boolean(value),

        Expression::Prefix {  
            operator, 
            right ,
            ..
        } => {
            let right_value = eval_expression(
                *right,
                env,
            );
            if right_value.is_error() {
                return right_value;
            }
            eval_prefix_expression(operator, right_value)
        },

        Expression::Infix {
            left,
            operator,
            right,
            ..
        } => {
            let left_value = eval_expression(
                *left,
                env,
            );
            if left_value.is_error() {
                return left_value;
            }
            let right_value = eval_expression(
                *right,
                env,
            );
            if right_value.is_error() {
                return right_value;
            }
            eval_infix_expression(operator, left_value, right_value)
        },

        Expression::If { 
            condition, 
            consequence, 
            alternative,
            ..
        } => {
            eval_if_expression(
                *condition, 
                consequence, 
                alternative,
                env,
            )
        }

        _ => Object::Null
    }
}

fn eval_identifier(
    identifier: ast::Identifier, 
    env: &mut Environment
) -> Object {
    match env.get(&identifier.value) {
        Some(value) => value,
        None => Object::Error(format!("identifier not found: {}", identifier.value))
    }
}

fn eval_prefix_expression(operator: String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => Object::Error(format!("unknown operator: {}{}", operator, right.type_of()))
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

fn eval_minus_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(value) => {
            Object::Integer(-value)
        }
        _ => Object::Error(
            format!(
                "unknown operator: -{}", 
                right.type_of()
            )
        )
    }
}

fn eval_infix_expression(operator: String, left: Object, right: Object) -> Object {
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
            if &left.type_of() != &right.type_of() {
                return Object::Error(format!(
                    "type mismatch: {} {} {}", 
                    left.type_of(), 
                    operator, 
                    right.type_of()
                ));
            }
            
            return Object::Error(format!(
                "unknown operator: {} {} {}", 
                left.type_of(), 
                operator, 
                right.type_of()
            ));
        }
    }

}

fn eval_integer_infix_expression(operator: String, left: i64, right: i64) -> Object {
    match operator.as_str() {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        ">" => Object::Boolean(left > right),
        "<" => Object::Boolean(left < right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Error(
            format!(
                "unknown operator: INTEGER {} INTEGER",
                operator,
            )
        )
    }
}

fn eval_boolean_infix_expression(operator: String, left: bool, right: bool) -> Object {
    match operator.as_str() {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Error(
            format!(
                "unknown operator: BOOLEAN {} BOOLEAN",
                operator,
            )
        )
    }
}

fn eval_if_expression(
    condition: Expression, 
    consequence: ast::BlockStatement, 
    alternative: Option<ast::BlockStatement>,
    env: &mut Environment,
) -> Object {
    let condition_value = eval_expression(
        condition,
        env,
    );
    
    if condition_value.is_error() {
        return condition_value;
    }

    if is_truthy(condition_value) {
        return eval_block_statement(consequence, env);
    } else if alternative.is_some() {
        return eval_block_statement(alternative.unwrap(), env);
    } else {
        return Object::Null;
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Boolean(value) => value,
        Object::Null => false,
        _ => true
    }
}

#[cfg(test)]
mod tests;