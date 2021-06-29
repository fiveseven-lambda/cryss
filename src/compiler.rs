//! 抽象構文木を型チェックして実行可能な式にする．

use crate::{error, pos, program, syntax, value};
use std::collections::HashMap;

use std::cell::Cell;
use std::rc::Rc;

// Rc は，関数の中身をメモリ上に置いておくのに用いる．

// グローバル変数は global: HashMap<String, Value> として保管
// Value には RcCell<f64>, RcCell<bool> などが入る．関数も入る
// compile 時にこれらは Reference の形で式の中に入る

// syntax::Statement::Expression
// compile するときに global を参照，実行．
// syntax::Statement::Substitution
// lhs は global の参照にする
// syntax::Statement::Declaration
// global に対して変数を insert し，その参照として Substitution を実行

// syntax::Statement::Block
// global を clone して local: HashMap を作る．
// Expression, Substitution では local を参照
// Declaration では local に対して変数を insert する．

// 引数 Vec<Value>
// オプション引数 HashMap<String, Value>
// ローカル変数 Vec<Value>
// そしてこれらは HashMap<String, Value> にも入れておく（コンパイル後は消える）

fn compile_node(
    range: &pos::Range,
    node: syntax::Node,
    variables: &HashMap<String, value::Value>,
) -> Result<program::Expression, error::Error> {
    Ok(match node {
        syntax::Node::Identifier(name) => match variables
            .get(&name)
            .ok_or(error::Error::UndefinedVariable(name, range.clone()))?
        {
            value::Value::Real(rc) => {
                program::Expression::Real(program::RealExpression::Reference(rc.clone()))
            }
            value::Value::Boolean(rc) => {
                program::Expression::Boolean(program::BooleanExpression::Reference(rc.clone()))
            }
            value::Value::Sound(rc) => {
                program::Expression::Sound(program::SoundExpression::Reference(rc.clone()))
            }
            value::Value::String(rc) => {
                program::Expression::String(program::StringExpression::Reference(rc.clone()))
            }
            value::Value::RealFunction(fnc) => program::Expression::RealFunction(fnc.clone()),
            value::Value::BooleanFunction(fnc) => program::Expression::BooleanFunction(fnc.clone()),
            value::Value::SoundFunction(fnc) => program::Expression::SoundFunction(fnc.clone()),
            value::Value::StringFunction(fnc) => program::Expression::StringFunction(fnc.clone()),
            value::Value::VoidFunction(fnc) => program::Expression::VoidFunction(fnc.clone()),
        },
        syntax::Node::Parameter(_) => todo!(),
        syntax::Node::Number(value) => {
            program::Expression::Real(program::RealExpression::Const(value))
        }
        syntax::Node::String(value) => {
            program::Expression::String(program::StringExpression::Const(value))
        }
        syntax::Node::Print(range, node) => match compile_node(&range, *node, variables)? {
            program::Expression::Real(expr) => {
                program::Expression::Real(program::RealExpression::Print(expr.into()))
            }
            program::Expression::Boolean(expr) => {
                program::Expression::Boolean(program::BooleanExpression::Print(expr.into()))
            }
            program::Expression::Sound(expr) => {
                program::Expression::Sound(program::SoundExpression::Play(expr.into()))
            }
            program::Expression::String(expr) => {
                program::Expression::String(program::StringExpression::Print(expr.into()))
            }
            _ => return Err(error::Error::CannotPrint(range)),
        },
        syntax::Node::Minus(expr) => match expr.0 {
            Some((range, node)) => match compile_node(&range, node, variables)? {
                program::Expression::Real(expr) => {
                    program::Expression::Real(program::RealExpression::Minus(expr.into()))
                }
                program::Expression::Sound(expr) => {
                    program::Expression::Sound(program::SoundExpression::Minus(expr.into()))
                }
                _ => return Err(error::Error::TypeMismatchMinus(range)),
            },
            None => return Err(error::Error::EmptyOperandMinus(range.clone())),
        },
        syntax::Node::Reciprocal(expr) => match expr.0 {
            Some((range, node)) => match compile_node(&range, node, variables)? {
                program::Expression::Real(expr) => {
                    program::Expression::Real(program::RealExpression::Reciprocal(expr.into()))
                }
                program::Expression::Sound(expr) => {
                    program::Expression::Sound(program::SoundExpression::Reciprocal(expr.into()))
                }
                _ => return Err(error::Error::TypeMismatchReciprocal(range)),
            },
            None => return Err(error::Error::EmptyOperandReciprocal(range.clone())),
        },
        _ => todo!(),
    })
}
fn compile_expression(
    expression: syntax::Expression,
    variables: &HashMap<String, value::Value>,
) -> Result<Option<program::Expression>, error::Error> {
    Ok(match expression.0 {
        Some((range, node)) => Some(compile_node(&range, node, variables)?),
        None => None,
    })
}
pub fn compile_statement(
    statement: syntax::Statement,
    variables: &mut HashMap<String, value::Value>,
) -> Result<program::Statement, error::Error> {
    match statement {
        syntax::Statement::Expression(expression) => Ok(program::Statement::Expression(
            compile_expression(expression, variables)?,
        )),
        _ => todo!(),
    }
}
