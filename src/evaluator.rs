use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{self, Statement, Expression};
use crate::object::{Object, Environment, EvalError};

mod infix;
mod prefix;
mod ident;

use ident::eval_identifier;
use prefix::eval_prefix_expression;
use infix::eval_infix_expression;



pub fn eval_program(
    program: &ast::Program,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, EvalError> {
    
    let mut value = Object::Null;

    for stmt in &program.statements {
        let result  = eval_stmt(stmt, Rc::clone(&env));

        match result {
            Ok(val) => {
                if let Object::ReturnValue(ret_val) = val {
                    return Ok(*ret_val);
                }
                value = val;
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    return Ok(value);
}

pub fn eval_block_statement(
    block: &ast::BlockStatement,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, EvalError> {
    let mut value = Object::Null;

    for stmt in &block.statements {
        let result = eval_stmt(stmt, Rc::clone(&env));

        match result {
            Ok(val) => {
                if !val.is_null() && val.is_return() {
                    return Ok(val);
                }
                value = val;
            },
            Err(err) => {
                return Err(err);
            }
        }
    }

    return Ok(value);
}


pub fn eval_stmt(
    stmt: &Statement,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, EvalError> {
    match stmt {
        Statement::Expression {
            expression,
            ..
        } => {
            eval_expression(
                expression,
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
                        value,
                        env,
                    );
                    val
                },
                None => Ok(
                    Object::ReturnValue(Box::new(Object::Null))
                )
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
            let result = eval_expression(
                value,
                Rc::clone(&env),
            );
            match result {
                Ok(val) => {
                    // Save a: 5
                    env.borrow_mut().set(
                        name.value.clone(), 
                        val.clone(),
                    );
                    return Ok(val);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}

pub fn eval_expression(
    expression: &Expression,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, EvalError> {
    match expression {
        Expression::Ident(identifier) => {
            eval_identifier(identifier, env)
        }
        Expression::IntegerLiteral { 
            value ,
            ..
        } => {
            Ok(Object::Integer(*value))
        }

        Expression::Boolean { 
            value ,
            ..
        } => {
            Ok(Object::Boolean(*value))
        },

        Expression::StringLiteral {
            value,
            ..
        } => {
            Ok(Object::String(value.clone()))
        }

        Expression::Prefix {  
            operator, 
            right ,
            ..
        } => {
            let right_value = eval_expression(
                right,
                env,
            );
            
            match right_value {
                Ok(value) => eval_prefix_expression(
                    operator.clone(), 
                    value
                ),
                Err(_) => right_value,
            }
        },

        Expression::Infix {
            left,
            operator,
            right,
            ..
        } => {
            let left_value = eval_expression(
                left,
                Rc::clone(&env),
            );
            if left_value.is_err() {
                return left_value;
            }
            let right_value = eval_expression(
                right,
                Rc::clone(&env),
            );
            if right_value.is_err() {
                return right_value;
            }
            eval_infix_expression(
                operator.clone(), 
                left_value.unwrap(), 
                right_value.unwrap(),
            )
        },

        Expression::If { 
            condition, 
            consequence, 
            alternative,
            ..
        } => {
            eval_if_expression(
                condition, 
                consequence, 
                alternative,
                env,
            )
        }

        Expression::FunctionLiteral {
            parameters,
            body,
            ..
        } => {
            let function = Object::Function {
                parameters: parameters.clone(),
                body: Rc::clone(body),
                env: Rc::clone(&env),
            };
            Ok(function)
        }

        Expression::Call {
            function,
            arguments,
            ..
        } => {
            let function_result = eval_expression(
                function,
                Rc::clone(&env),
            );
            if function_result.is_err() {
                return function_result;
            }
            let function = function_result.unwrap();
            let args_result = eval_arguments(
                arguments,
                Rc::clone(&env),
            );
            match args_result {
                Ok(args) => {
                    apply_function(function, args)
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

    }
}


fn eval_arguments(
    expressions: &Vec<Expression>,
    env: Rc<RefCell<Environment>>,
) -> Result<Vec<Object>, EvalError> {
    let mut args = vec![];

    for expression in expressions {
        let evaluated = eval_expression(
            expression,
            Rc::clone(&env),
        );
        match evaluated {
            Ok(val) => {
                args.push(val);
            },
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(args)
}

fn apply_function(
    function: Object,
    args: Vec<Object>,
) -> Result<Object, EvalError> {
    match function {
        Object::Function {
            parameters,
            body,
            env: function_env,
        } => {
            let extended_env = extend_function_env(
                function_env,
                parameters,
                args,
            );
            eval_block_statement(&body, extended_env)
        },

        Object::Builtin(builtin) => {
            builtin(args)
        },

        _ => {
            return Err(EvalError::UnknownFunction(
                function.type_name().to_string(),
            ));
        }
    }
}

fn extend_function_env(
    env: Rc<RefCell<Environment>>,
    parameters: Vec<ast::Identifier>,
    args: Vec<Object>,
) -> Rc<RefCell<Environment>> {
    let function_env = Environment::new_enclosed(
        Rc::clone(&env),
    );
    for (i, param) in parameters.iter().enumerate() {
        function_env.borrow_mut().set(
            param.value.clone(),
            args[i].clone(),
        )
    }

    function_env
}

fn eval_if_expression(
    condition: &Expression, 
    consequence: &ast::BlockStatement, 
    alternative: &Option<Rc<ast::BlockStatement>>,
    env: Rc<RefCell<Environment>>,
) -> Result<Object, EvalError> {
    let condition_result = eval_expression(
        condition,
        Rc::clone(&env),
    )?;

    if is_truthy(condition_result) {
        return eval_block_statement(
            consequence, 
            env
        );
    } else {
        match alternative {
            Some(alt) => {
                return eval_block_statement(
                    alt, 
                    env
                );
            }
            None => {
                return Ok(Object::Null);
            }
        }
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