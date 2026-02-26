use std::fmt;
use super::{Node, Program};


impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(stmt) = self.statements.first() {
            stmt.token_literal()
        } else {
            "".to_string()
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}