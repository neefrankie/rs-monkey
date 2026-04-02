use std::cell::RefCell;
use std::rc::Rc;

use crate::object::{Object, Environment, EvalError};
use crate::ast;

pub(super) fn eval_identifier(
    identifier: &ast::Identifier, 
    env: Rc<RefCell<Environment>>,
) -> Result<Object, EvalError> {
    match env.borrow().get(&identifier.value) {
        Some(value) => Ok(value.clone()),
        None => Err(EvalError::UnknownIdentifier(
            identifier.value.clone()
        )),
    }
}