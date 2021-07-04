//! 抽象構文木（ `mod syntax` ）を型チェックして実行可能なプログラム（ `mod program` ）にする．

use crate::{error, pos, program, syntax, types, value};
use std::collections::HashMap;

use std::cell::Cell;
use std::rc::Rc;

/// variable は，そのスコープに存在する変数
fn compile_expression(
    expression: syntax::Expression,
    variables: &HashMap<String, value::Value>,
) -> Result<(program::Expression, pos::Range), error::Error> {
    use program::Expression::{Boolean, Real, Sound, String, Void};
    use program::{
        BooleanExpression, RealExpression, SoundExpression, StringExpression, VoidExpression,
    };
    use syntax::Node;
    use value::Value;

    let ret = match expression.node {
        Node::Identifier(name) => match variables.get(&name) {
            Some(Value::Real(rc)) => RealExpression::Reference(rc.clone()).into(),
            Some(Value::Boolean(rc)) => BooleanExpression::Reference(rc.clone()).into(),
            Some(Value::Sound(rc)) => SoundExpression::Reference(rc.clone()).into(),
            Some(Value::String(rc)) => StringExpression::Reference(rc.clone()).into(),
            None => return Err(error::Error::UndefinedVariable(name, expression.range)),
        },
        Node::Invocation(_, _, _) => todo!(),
        Node::Parameter(_) => todo!(),
        Node::Number(value) => RealExpression::Const(value).into(),
        Node::String(string) => StringExpression::Const(string).into(),
        Node::Print(expr) => match compile_expression(*expr, variables)? {
            (Real(expr), _) => RealExpression::Print(expr.into()).into(),
            (Boolean(expr), _) => BooleanExpression::Print(expr.into()).into(),
            (Sound(expr), _) => SoundExpression::Play(expr.into()).into(),
            (String(expr), _) => StringExpression::Print(expr.into()).into(),
            (other, range) => return Err(error::Error::TypeMismatchUnary(range, other.ty())),
        },
        Node::Minus(expr) => match compile_expression(*expr, variables)? {
            (Real(expr), _) => RealExpression::Minus(expr.into()).into(),
            (Sound(expr), _) => SoundExpression::Minus(expr.into()).into(),
            (other, range) => return Err(error::Error::TypeMismatchUnary(range, other.ty())),
        },
        Node::Reciprocal(expr) => match compile_expression(*expr, variables)? {
            (Real(expr), _) => RealExpression::Reciprocal(expr.into()).into(),
            (Sound(expr), _) => SoundExpression::Reciprocal(expr.into()).into(),
            (other, range) => return Err(error::Error::TypeMismatchUnary(range, other.ty())),
        },
        Node::Not(expr) => match compile_expression(*expr, variables)? {
            (Boolean(expr), _) => BooleanExpression::Not(expr.into()).into(),
            (other, range) => return Err(error::Error::TypeMismatchUnary(range, other.ty())),
        },
        Node::Add(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                RealExpression::Add(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Sound(right), _)) => {
                SoundExpression::Add(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::Add(left.into(), SoundExpression::Real(right).into()).into()
            }
            ((Real(left), _), (Sound(right), _)) => {
                SoundExpression::Add(SoundExpression::Real(left).into(), right.into()).into()
            }
            ((String(left), _), (String(right), _)) => {
                StringExpression::Add(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Sub(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                RealExpression::Sub(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Sound(right), _)) => {
                SoundExpression::Sub(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::Sub(left.into(), SoundExpression::Real(right).into()).into()
            }
            ((Real(left), _), (Sound(right), _)) => {
                SoundExpression::Sub(SoundExpression::Real(left).into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Mul(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                RealExpression::Mul(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Sound(right), _)) => {
                SoundExpression::Mul(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::Mul(left.into(), SoundExpression::Real(right).into()).into()
            }
            ((Real(left), _), (Sound(right), _)) => {
                SoundExpression::Mul(SoundExpression::Real(left).into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Div(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                RealExpression::Div(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Sound(right), _)) => {
                SoundExpression::Div(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::Div(left.into(), SoundExpression::Real(right).into()).into()
            }
            ((Real(left), _), (Sound(right), _)) => {
                SoundExpression::Div(SoundExpression::Real(left).into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Rem(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                RealExpression::Rem(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Sound(right), _)) => {
                SoundExpression::Rem(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::Rem(left.into(), SoundExpression::Real(right).into()).into()
            }
            ((Real(left), _), (Sound(right), _)) => {
                SoundExpression::Rem(SoundExpression::Real(left).into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Pow(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                RealExpression::Pow(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Sound(right), _)) => {
                SoundExpression::Pow(left.into(), right.into()).into()
            }
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::Pow(left.into(), SoundExpression::Real(right).into()).into()
            }
            ((Real(left), _), (Sound(right), _)) => {
                SoundExpression::Pow(SoundExpression::Real(left).into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::LeftShift(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::LeftShift(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::RightShift(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::RightShift(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Less(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                BooleanExpression::RealLess(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Greater(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                BooleanExpression::RealGreater(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Equal(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                BooleanExpression::RealEqual(left.into(), right.into()).into()
            }
            ((String(left), _), (String(right), _)) => {
                BooleanExpression::StringEqual(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::NotEqual(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                BooleanExpression::RealNotEqual(left.into(), right.into()).into()
            }
            ((String(left), _), (String(right), _)) => {
                BooleanExpression::StringNotEqual(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::And(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Boolean(left), _), (Boolean(right), _)) => {
                BooleanExpression::And(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Or(left, right) => match (
            compile_expression(*left, variables)?,
            compile_expression(*right, variables)?,
        ) {
            ((Boolean(left), _), (Boolean(right), _)) => {
                BooleanExpression::Or(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(error::Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Group(expr) => compile_expression(*expr, variables)?.0,
        Node::Score(_) => todo!(),
    };
    Ok((ret, expression.range))
}

pub fn compile_statement(
    statement: syntax::Statement,
    variables: &mut HashMap<String, value::Value>,
) -> Result<program::Statement, error::Error> {
    Ok(match statement {
        syntax::Statement::Expression(expr) => program::Statement::Expression(
            expr.map(|expr| compile_expression(expr, variables))
                .transpose()?
                .map(|(expr, _)| expr),
        ),
        syntax::Statement::Substitution(range, name, expr) => {
            let lhs = match variables.get(&name) {
                Some(rc) => rc,
                None => return Err(error::Error::UndefinedVariable(name, range)),
            };
            let rhs = compile_expression(expr, variables)?;
            match (lhs, rhs.0) {
                (value::Value::Real(rc), program::Expression::Real(expr)) => {
                    program::Statement::RealSubstitution(rc.clone(), expr)
                }
                (value::Value::Boolean(rc), program::Expression::Boolean(expr)) => {
                    program::Statement::BooleanSubstitution(rc.clone(), expr)
                }
                (value::Value::Sound(rc), program::Expression::Sound(expr)) => {
                    program::Statement::SoundSubstitution(rc.clone(), expr)
                }
                (value::Value::String(rc), program::Expression::String(expr)) => {
                    program::Statement::StringSubstitution(rc.clone(), expr)
                }
                (_, r) => {
                    return Err(error::Error::TypeMismatchBinary(
                        range,
                        lhs.ty(),
                        rhs.1,
                        r.ty(),
                    ))
                }
            }
        }
        syntax::Statement::Declaration(range, name, expr) => {
            let rhs = compile_expression(expr, variables)?;
            match rhs.0 {
                program::Expression::Real(expr) => {
                    let rc = Rc::new(Cell::new(0.));
                    variables.insert(name, value::Value::Real(rc.clone()));
                    program::Statement::RealSubstitution(rc, expr)
                }
                program::Expression::Boolean(expr) => {
                    let rc = Rc::new(Cell::new(false));
                    variables.insert(name, value::Value::Boolean(rc.clone()));
                    program::Statement::BooleanSubstitution(rc, expr)
                }
                program::Expression::String(expr) => {
                    let rc = Rc::new(Cell::new("".to_string()));
                    variables.insert(name, value::Value::String(rc.clone()));
                    program::Statement::StringSubstitution(rc, expr)
                }
                program::Expression::Sound(expr) => {
                    todo!();
                }
                program::Expression::Void(expr) => {
                    return Err(error::Error::VoidRHS(range));
                }
            }
        }
        _ => todo!(),
    })
}
