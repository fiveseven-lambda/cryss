//! プログラム（ `mod program` ）を実行する環境

use crate::{compiler, error, syntax, value};
use std::collections::HashMap;

use std::cell::Cell;
use std::rc::Rc;

pub struct Environment {
    variables: HashMap<String, value::Value>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut variables = HashMap::new();
        variables.insert(
            "true".to_string(),
            value::Value::Boolean(Rc::new(Cell::new(true))),
        );
        variables.insert(
            "false".to_string(),
            value::Value::Boolean(Rc::new(Cell::new(false))),
        );
        Environment { variables }
    }
    pub fn run(&mut self, statement: syntax::Statement) -> Result<(), error::Error> {
        compiler::compile_statement(statement, &mut self.variables)?.run();
        Ok(())
    }
}
