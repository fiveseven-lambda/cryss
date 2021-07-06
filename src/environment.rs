//! プログラム（ `mod program` ）を実行する環境

use crate::compiler;
use crate::error::Error;
use crate::function::Function;
use crate::syntax::Statement;
use crate::value::Value;
use std::collections::HashMap;

use std::cell::Cell;
use std::rc::Rc;

pub struct Environment {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut variables = HashMap::new();
        variables.insert("true".to_string(), Value::Boolean(Rc::new(Cell::new(true))));
        variables.insert(
            "false".to_string(),
            Value::Boolean(Rc::new(Cell::new(false))),
        );
        variables.insert(
            "PI".to_string(),
            Value::Real(Rc::new(Cell::new(std::f64::consts::PI))),
        );
        variables.insert(
            "E".to_string(),
            Value::Real(Rc::new(Cell::new(std::f64::consts::PI))),
        );
        let mut functions = HashMap::new();
        functions.insert("sqrt".to_string(), Function::primitive_real_1(f64::sqrt));
        functions.insert("sin".to_string(), Function::primitive_real_1(f64::sin));
        functions.insert("cos".to_string(), Function::primitive_real_1(f64::cos));
        functions.insert("tan".to_string(), Function::primitive_real_1(f64::tan));
        functions.insert("exp".to_string(), Function::primitive_real_1(f64::exp));
        functions.insert("log".to_string(), Function::primitive_real_1(f64::ln));
        functions.insert("max".to_string(), Function::primitive_real_2(f64::max));
        functions.insert("min".to_string(), Function::primitive_real_2(f64::min));
        functions.insert("Sin".to_string(), Function::sin());
        functions.insert("Linear".to_string(), Function::linear());
        functions.insert("Exp".to_string(), Function::exp());
        functions.insert("write".to_string(), Function::write());
        Environment {
            variables,
            functions,
        }
    }
    pub fn run(&mut self, statement: Statement) -> Result<(), Error> {
        compiler::compile_statement(statement, &mut self.variables, &mut self.functions)?.run();
        Ok(())
    }
}
