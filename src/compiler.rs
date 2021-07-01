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

macro_rules! def_binary_operator_real {
    ($left:ident, $right:ident, $range:ident, $variables:ident, $expected:expr; $rop:ident; $($lt:path, $rt:path => $ot:ident: $out:ident :: $op:ident);* $(;)?) => {{
        let (left_range, left) = $left.0.ok_or(error::Error::EmptyOperand($range.clone()))?;
        let (right_range, right) = $right.0.ok_or(error::Error::EmptyOperand($range.clone()))?;
        let left = compile_node(&left_range, left, $variables)?;
        let right = compile_node(&right_range, right, $variables)?;
        match (left, right) {
            (Real(left), Real(right)) => Real(RealExpression::$rop(left.into(), right.into())),
            (Sound(left), Sound(right)) => Sound(SoundExpression::$rop(left.into(), right.into())),
            (Sound(left), Real(right)) => Sound(SoundExpression::$rop(
                left.into(),
                SoundExpression::Real(right).into(),
            )),
            (Real(left), Sound(right)) => Sound(SoundExpression::$rop(
                SoundExpression::Real(left).into(),
                right.into(),
            )),
            $(($lt(left), $rt(right)) => $ot($out::$op(left.into(), right.into())),)*
            _ => {
                return Err(error::Error::TypeMismatchBinary(
                    left_range,
                    right_range,
                    $expected
                ))
            }
        }
    }};
}
macro_rules! def_binary_operator {
    ($left:ident, $right:ident, $range:ident, $variables:ident, $expected:expr; $($lt:path, $rt:path => $ot:ident: $out:ident :: $op:ident);* $(;)?) => {{
        let (left_range, left) = $left.0.ok_or(error::Error::EmptyOperand($range.clone()))?;
        let (right_range, right) = $right.0.ok_or(error::Error::EmptyOperand($range.clone()))?;
        let left = compile_node(&left_range, left, $variables)?;
        let right = compile_node(&right_range, right, $variables)?;
        match (left, right) {
            $(($lt(left), $rt(right)) => $ot($out::$op(left.into(), right.into())),)*
            _ => {
                return Err(error::Error::TypeMismatchBinary(
                    left_range,
                    right_range,
                    $expected
                ))
            }
        }
    }};
}

/// variable は，
/// そのスコープに存在する変数
fn compile_node(
    range: &pos::Range,
    node: syntax::Node,
    variables: &HashMap<String, value::Value>,
) -> Result<program::Expression, error::Error> {
    use program::{
        BooleanExpression, Expression, RealExpression, SoundExpression, StringExpression,
    };
    use syntax::Node;
    use Expression::{Boolean, Real, Sound, String};
    Ok(match node {
        syntax::Node::Identifier(name) => match variables
            .get(&name)
            .ok_or(error::Error::UndefinedVariable(name, range.clone()))?
        {
            value::Value::Real(rc) => Real(RealExpression::Reference(rc.clone())),
            value::Value::Boolean(rc) => Boolean(BooleanExpression::Reference(rc.clone())),
            value::Value::Sound(rc) => Sound(SoundExpression::Reference(rc.clone())),
            value::Value::String(rc) => String(StringExpression::Reference(rc.clone())),
            value::Value::RealFunction(fnc) => Expression::RealFunction(fnc.clone()),
            value::Value::BooleanFunction(fnc) => Expression::BooleanFunction(fnc.clone()),
            value::Value::SoundFunction(fnc) => Expression::SoundFunction(fnc.clone()),
            value::Value::StringFunction(fnc) => Expression::StringFunction(fnc.clone()),
            value::Value::VoidFunction(fnc) => Expression::VoidFunction(fnc.clone()),
        },
        Node::Parameter(_) => todo!(),
        Node::Number(value) => Real(RealExpression::Const(value)),
        Node::String(value) => String(StringExpression::Const(value)),
        Node::Print(range, node) => match compile_node(&range, *node, variables)? {
            Real(expr) => Real(RealExpression::Print(expr.into())),
            Boolean(expr) => Boolean(BooleanExpression::Print(expr.into())),
            Sound(expr) => Sound(SoundExpression::Play(expr.into())),
            String(expr) => String(StringExpression::Print(expr.into())),
            _ => return Err(error::Error::CannotPrint(range)),
        },
        Node::Minus(expr) => {
            let (range, node) = expr.0.ok_or(error::Error::EmptyOperand(range.clone()))?;
            match compile_node(&range, node, variables)? {
                Real(expr) => Real(RealExpression::Minus(expr.into())),
                Sound(expr) => Sound(SoundExpression::Minus(expr.into())),
                _ => return Err(error::Error::TypeMismatchReal(range)),
            }
        }
        Node::Reciprocal(expr) => {
            let (range, node) = expr.0.ok_or(error::Error::EmptyOperand(range.clone()))?;
            match compile_node(&range, node, variables)? {
                Real(expr) => Real(RealExpression::Reciprocal(expr.into())),
                Sound(expr) => Sound(SoundExpression::Reciprocal(expr.into())),
                _ => return Err(error::Error::TypeMismatchReal(range)),
            }
        }
        Node::Not(expr) => {
            let (range, node) = expr.0.ok_or(error::Error::EmptyOperand(range.clone()))?;
            match compile_node(&range, node, variables)? {
                Boolean(expr) => Boolean(BooleanExpression::Not(expr.into())),
                _ => return Err(error::Error::TypeMismatchBoolean(range)),
            }
        }
        Node::Add(left, right) => def_binary_operator_real! {
            left, right, range, variables, "real, Sound or string";
            Add;
            String, String => String: StringExpression::Add;
        },
        Node::Sub(left, right) => def_binary_operator_real! {
            left, right, range, variables, "real or Sound";
            Sub;
        },
        Node::Mul(left, right) => def_binary_operator_real! {
            left, right, range, variables, "real or Sound";
            Mul;
        },
        Node::Div(left, right) => def_binary_operator_real! {
            left, right, range, variables, "real or Sound";
            Div;
        },
        Node::Rem(left, right) => def_binary_operator_real! {
            left, right, range, variables, "real or Sound";
            Rem;
        },
        Node::Pow(left, right) => def_binary_operator_real! {
            left, right, range, variables, "real or Sound";
            Pow;
        },
        Node::LeftShift(left, right) => def_binary_operator! {
            left, right, range, variables, "Sound (left) and real (right)";
            Sound, Real => Sound: SoundExpression::LeftShift;
        },
        Node::RightShift(left, right) => def_binary_operator! {
            left, right, range, variables, "Sound (left) and real (right)";
            Sound, Real => Sound: SoundExpression::RightShift;
        },
        Node::Less(left, right) => def_binary_operator! {
            left, right, range, variables, "real";
            Real, Real => Boolean: BooleanExpression::RealLess;
        },
        Node::Greater(left, right) => def_binary_operator! {
            left, right, range, variables, "real";
            Real, Real => Boolean: BooleanExpression::RealGreater;
        },
        Node::Equal(left, right) => def_binary_operator! {
            left, right, range, variables, "real or string";
            Real, Real => Boolean: BooleanExpression::RealEqual;
            String, String => Boolean: BooleanExpression::StringEqual;
        },
        Node::NotEqual(left, right) => def_binary_operator! {
            left, right, range, variables, "real or string";
            Real, Real => Boolean: BooleanExpression::RealNotEqual;
            String, String => Boolean: BooleanExpression::StringNotEqual;
        },
        Node::And(left, right) => def_binary_operator! {
            left, right, range, variables, "boolean";
            Boolean, Boolean => Boolean: BooleanExpression::And;
        },
        Node::Or(left, right) => def_binary_operator! {
            left, right, range, variables, "boolean";
            Boolean, Boolean => Boolean: BooleanExpression::Or;
        },
        Node::Group(expr) => {
            let (range, node) = expr.0.ok_or(error::Error::EmptyOperand(range.clone()))?;
            return compile_node(&range, node, variables);
        }
        Node::Invocation(_, _, _, _) => todo!(),
        Node::Score(_) => todo!(),
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
