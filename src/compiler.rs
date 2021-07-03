//! 抽象構文木（ `mod syntax` ）を型チェックして実行可能なプログラム（ `mod program` ）にする．

use crate::{error, pos, program, syntax, value};
use std::collections::HashMap;

use std::cell::Cell;
use std::rc::Rc;

/// variable は，
/// そのスコープに存在する変数
fn compile_expression(
    expression: syntax::Expression,
    variables: &HashMap<String, value::Value>,
) -> Result<program::Expression, error::Error> {
    match expression.node {
        syntax::Node::Identifier(name) => match variables
            .get(&name)
            .ok_or(error::Error::UndefinedVariable(name, expression.range))?
        {
            value::Value::Real(rc) => Ok(program::RealExpression::Reference(rc.clone()).into()),
            value::Value::Boolean(rc) => {
                Ok(program::BooleanExpression::Reference(rc.clone()).into())
            }
            value::Value::Sound(rc) => Ok(program::SoundExpression::Reference(rc.clone()).into()),
            value::Value::String(rc) => Ok(program::StringExpression::Reference(rc.clone()).into()),
        },
        _ => todo!(),
    }
}
pub fn compile_statement(
    statement: syntax::Statement,
    variables: &mut HashMap<String, value::Value>,
) -> Result<program::Statement, error::Error> {
    todo!();
}
