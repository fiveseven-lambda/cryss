//! プログラム（ `mod program` ）を実行する環境

use crate::{program, value};
use std::collections::HashMap;

struct Environment {
    variables: HashMap<String, value::Value>,
}

impl Environment {
    fn new() -> Environment {
        Environment {
            variables: HashMap::new(),
        }
    }
    fn run(&mut self, statement: program::Statement) {}
}

/*
 * 要るか？これ
pub trait Expression {
    type Output;
    fn evaluate(&self) -> Self::Output;
}
impl Expression for RealExpression {
    type Output = f64;
    fn evaluate(&self) -> f64 {
        todo!();
    }
}
impl Expression for BooleanExpression {
    type Output = bool;
    fn evaluate(&self) -> bool {
        todo!();
    }
}
impl Expression for SoundExpression {
    type Output = sound::Sound;
    fn evaluate(&self) -> sound::Sound {
        todo!();
    }
}
impl Expression for StringExpression {
    type Output = String;
    fn evaluate(&self) -> String {
        todo!();
    }
}
impl Expression for VoidExpression {
    type Output = ();
    fn evaluate(&self) {
        todo!();
    }
}
*/
