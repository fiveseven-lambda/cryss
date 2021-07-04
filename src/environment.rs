//! プログラム（ `mod program` ）を実行する環境

use crate::{compiler, error, syntax, value};
use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, value::Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            variables: HashMap::new(),
        }
    }
    pub fn run(&mut self, statement: syntax::Statement) -> Result<(), error::Error> {
        compiler::compile_statement(statement, &mut self.variables)?.run();
        Ok(())
    }
}
