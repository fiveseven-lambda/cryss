//! 抽象構文木（ `mod syntax` ）を型チェックして実行可能なプログラム（ `mod program` ）にする．

use crate::{error, function, pos, program, sound, syntax, value};
use std::collections::HashMap;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// variable は，そのスコープに存在する変数
fn compile_expression(
    expression: syntax::Expression,
    variables: &HashMap<String, value::Value>,
    functions: &HashMap<String, function::Function>,
) -> Result<(program::Expression, pos::Range), error::Error> {
    use error::Error;
    use program::Expression::{Boolean, Real, Sound, String, Void};
    use program::{
        Argument, BooleanExpression, RealExpression, SoundExpression, StringExpression,
        VoidExpression,
    };
    use syntax::Node;
    use value::Value;

    let ret = match expression.node {
        Node::Identifier(name) => match variables.get(&name) {
            Some(Value::Real(rc)) => RealExpression::Reference(rc.clone()).into(),
            Some(Value::Boolean(rc)) => BooleanExpression::Reference(rc.clone()).into(),
            Some(Value::Sound(rc)) => SoundExpression::Reference(rc.clone()).into(),
            Some(Value::String(rc)) => StringExpression::Reference(rc.clone()).into(),
            None => return Err(Error::UndefinedVariable(name, expression.range)),
        },
        Node::Invocation(name, arguments, named_arguments) => {
            let function = match functions.get(&name) {
                Some(function) => function,
                None => return Err(Error::UndefinedFunction(name, expression.range)),
            };
            let mut vec = Vec::new();
            let mut sounds = Vec::new();
            if arguments.len() != function.arguments.len() {
                return Err(Error::WrongNumberOfArguments(
                    expression.range,
                    function.arguments.len(),
                    arguments.len(),
                ));
            }
            for (expr, expected) in arguments.into_iter().zip(&function.arguments) {
                let argument = compile_expression(expr, variables, functions)?;
                match (expected, argument.0) {
                    (Value::Real(rc), Real(expr)) => vec.push(Argument::Real(rc.clone(), expr)),
                    (Value::Boolean(rc), Boolean(expr)) => {
                        vec.push(Argument::Boolean(rc.clone(), expr))
                    }
                    (Value::Sound(rc), Sound(expr)) => vec.push(Argument::Sound(rc.clone(), expr)),
                    (Value::String(rc), String(expr)) => {
                        vec.push(Argument::String(rc.clone(), expr))
                    }
                    (Value::Real(rc), Sound(expr)) => {
                        sounds.push((rc.clone(), expr));
                    }
                    (_, other) => return Err(Error::TypeMismatchArgument(argument.1, other.ty())),
                }
            }
            for (name, expr) in named_arguments {
                // todo: 名前付き引数も……
            }

            if sounds.is_empty() {
                match &function.body {
                    function::Body::Real(body) => {
                        RealExpression::Invocation(body.clone(), vec).into()
                    }
                    function::Body::Sound(body) => {
                        SoundExpression::Invocation(body.clone(), vec).into()
                    }
                    function::Body::Void(body) => {
                        VoidExpression::Invocation(body.clone(), vec).into()
                    }
                    _ => todo!(),
                }
            } else {
                match &function.body {
                    function::Body::Real(body) => {
                        SoundExpression::Apply(body.clone(), vec, sounds).into()
                    }
                    _ => todo!(),
                }
            }
        }
        Node::Parameter(_) => todo!(),
        Node::Number(value) => RealExpression::Const(value).into(),
        Node::String(string) => StringExpression::Const(string).into(),
        Node::Print(expr) => match compile_expression(*expr, variables, functions)? {
            (Real(expr), _) => RealExpression::Print(expr.into()).into(),
            (Boolean(expr), _) => BooleanExpression::Print(expr.into()).into(),
            (Sound(expr), _) => SoundExpression::Play(expr.into()).into(),
            (String(expr), _) => StringExpression::Print(expr.into()).into(),
            (other, range) => return Err(Error::TypeMismatchUnary(range, other.ty())),
        },
        Node::Minus(expr) => match compile_expression(*expr, variables, functions)? {
            (Real(expr), _) => RealExpression::Minus(expr.into()).into(),
            (Sound(expr), _) => SoundExpression::Minus(expr.into()).into(),
            (other, range) => return Err(Error::TypeMismatchUnary(range, other.ty())),
        },
        Node::Reciprocal(expr) => match compile_expression(*expr, variables, functions)? {
            (Real(expr), _) => RealExpression::Reciprocal(expr.into()).into(),
            (Sound(expr), _) => SoundExpression::Reciprocal(expr.into()).into(),
            (other, range) => return Err(Error::TypeMismatchUnary(range, other.ty())),
        },
        Node::Not(expr) => match compile_expression(*expr, variables, functions)? {
            (Boolean(expr), _) => BooleanExpression::Not(expr.into()).into(),
            (other, range) => return Err(Error::TypeMismatchUnary(range, other.ty())),
        },
        Node::Add(left, right) => {
            let l = compile_expression(*left, variables, functions)?;
            let r = compile_expression(*right, variables, functions)?;
            match (l.0, r.0) {
                (Real(left), Real(right)) => RealExpression::Add(left.into(), right.into()).into(),
                (Sound(left), Sound(right)) => {
                    SoundExpression::Add(left.into(), right.into()).into()
                }
                (Sound(left), Real(right)) => {
                    SoundExpression::Add(left.into(), SoundExpression::Real(right).into()).into()
                }
                (Real(left), Sound(right)) => {
                    SoundExpression::Add(SoundExpression::Real(left).into(), right.into()).into()
                }
                (String(left), String(right)) => {
                    StringExpression::Add(left.into(), right.into()).into()
                }
                (x, y) => return Err(Error::TypeMismatchBinary(l.1, x.ty(), r.1, y.ty())),
            }
        }
        Node::Sub(left, right) => {
            let l = compile_expression(*left, variables, functions)?;
            let r = compile_expression(*right, variables, functions)?;
            match (l.0, r.0) {
                (Real(left), Real(right)) => RealExpression::Sub(left.into(), right.into()).into(),
                (Sound(left), Sound(right)) => {
                    SoundExpression::Sub(left.into(), right.into()).into()
                }
                (Sound(left), Real(right)) => {
                    SoundExpression::Sub(left.into(), SoundExpression::Real(right).into()).into()
                }
                (Real(left), Sound(right)) => {
                    SoundExpression::Sub(SoundExpression::Real(left).into(), right.into()).into()
                }
                (x, y) => return Err(Error::TypeMismatchBinary(l.1, x.ty(), r.1, y.ty())),
            }
        }
        Node::Mul(left, right) => {
            let l = compile_expression(*left, variables, functions)?;
            let r = compile_expression(*right, variables, functions)?;
            match (l.0, r.0) {
                (Real(left), Real(right)) => RealExpression::Mul(left.into(), right.into()).into(),
                (Sound(left), Sound(right)) => {
                    SoundExpression::Mul(left.into(), right.into()).into()
                }
                (Sound(left), Real(right)) => {
                    SoundExpression::Mul(left.into(), SoundExpression::Real(right).into()).into()
                }
                (Real(left), Sound(right)) => {
                    SoundExpression::Mul(SoundExpression::Real(left).into(), right.into()).into()
                }
                (x, y) => return Err(Error::TypeMismatchBinary(l.1, x.ty(), r.1, y.ty())),
            }
        }
        Node::Div(left, right) => {
            let l = compile_expression(*left, variables, functions)?;
            let r = compile_expression(*right, variables, functions)?;
            match (l.0, r.0) {
                (Real(left), Real(right)) => RealExpression::Div(left.into(), right.into()).into(),
                (Sound(left), Sound(right)) => {
                    SoundExpression::Div(left.into(), right.into()).into()
                }
                (Sound(left), Real(right)) => {
                    SoundExpression::Div(left.into(), SoundExpression::Real(right).into()).into()
                }
                (Real(left), Sound(right)) => {
                    SoundExpression::Div(SoundExpression::Real(left).into(), right.into()).into()
                }
                (x, y) => return Err(Error::TypeMismatchBinary(l.1, x.ty(), r.1, y.ty())),
            }
        }
        Node::Rem(left, right) => {
            let l = compile_expression(*left, variables, functions)?;
            let r = compile_expression(*right, variables, functions)?;
            match (l.0, r.0) {
                (Real(left), Real(right)) => RealExpression::Rem(left.into(), right.into()).into(),
                (Sound(left), Sound(right)) => {
                    SoundExpression::Rem(left.into(), right.into()).into()
                }
                (Sound(left), Real(right)) => {
                    SoundExpression::Rem(left.into(), SoundExpression::Real(right).into()).into()
                }
                (Real(left), Sound(right)) => {
                    SoundExpression::Rem(SoundExpression::Real(left).into(), right.into()).into()
                }
                (x, y) => return Err(Error::TypeMismatchBinary(l.1, x.ty(), r.1, y.ty())),
            }
        }
        Node::Pow(left, right) => {
            let l = compile_expression(*left, variables, functions)?;
            let r = compile_expression(*right, variables, functions)?;
            match (l.0, r.0) {
                (Real(left), Real(right)) => RealExpression::Pow(left.into(), right.into()).into(),
                (Sound(left), Sound(right)) => {
                    SoundExpression::Pow(left.into(), right.into()).into()
                }
                (Sound(left), Real(right)) => {
                    SoundExpression::Pow(left.into(), SoundExpression::Real(right).into()).into()
                }
                (Real(left), Sound(right)) => {
                    SoundExpression::Pow(SoundExpression::Real(left).into(), right.into()).into()
                }
                (x, y) => return Err(Error::TypeMismatchBinary(l.1, x.ty(), r.1, y.ty())),
            }
        }
        Node::LeftShift(left, right) => match (
            compile_expression(*left, variables, functions)?,
            compile_expression(*right, variables, functions)?,
        ) {
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::LeftShift(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::RightShift(left, right) => match (
            compile_expression(*left, variables, functions)?,
            compile_expression(*right, variables, functions)?,
        ) {
            ((Sound(left), _), (Real(right), _)) => {
                SoundExpression::RightShift(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Less(left, right) => match (
            compile_expression(*left, variables, functions)?,
            compile_expression(*right, variables, functions)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                BooleanExpression::RealLess(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Greater(left, right) => match (
            compile_expression(*left, variables, functions)?,
            compile_expression(*right, variables, functions)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                BooleanExpression::RealGreater(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Equal(left, right) => match (
            compile_expression(*left, variables, functions)?,
            compile_expression(*right, variables, functions)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                BooleanExpression::RealEqual(left.into(), right.into()).into()
            }
            ((String(left), _), (String(right), _)) => {
                BooleanExpression::StringEqual(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::NotEqual(left, right) => match (
            compile_expression(*left, variables, functions)?,
            compile_expression(*right, variables, functions)?,
        ) {
            ((Real(left), _), (Real(right), _)) => {
                BooleanExpression::RealNotEqual(left.into(), right.into()).into()
            }
            ((String(left), _), (String(right), _)) => {
                BooleanExpression::StringNotEqual(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::And(left, right) => match (
            compile_expression(*left, variables, functions)?,
            compile_expression(*right, variables, functions)?,
        ) {
            ((Boolean(left), _), (Boolean(right), _)) => {
                BooleanExpression::And(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Or(left, right) => match (
            compile_expression(*left, variables, functions)?,
            compile_expression(*right, variables, functions)?,
        ) {
            ((Boolean(left), _), (Boolean(right), _)) => {
                BooleanExpression::Or(left.into(), right.into()).into()
            }
            ((l, x), (r, y)) => return Err(Error::TypeMismatchBinary(x, l.ty(), y, r.ty())),
        },
        Node::Group(expr) => compile_expression(*expr, variables, functions)?.0,
        Node::Score(_) => todo!(),
    };
    Ok((ret, expression.range))
}

pub fn compile_statement(
    statement: syntax::Statement,
    variables: &mut HashMap<String, value::Value>,
    functions: &mut HashMap<String, function::Function>,
) -> Result<program::Statement, error::Error> {
    use error::Error;
    Ok(match statement {
        syntax::Statement::Expression(expr) => program::Statement::Expression(
            expr.map(|expr| compile_expression(expr, variables, functions))
                .transpose()?
                .map(|(expr, _)| expr),
        ),
        syntax::Statement::Substitution(range, name, expr) => {
            let lhs = match variables.get(&name) {
                Some(rc) => rc,
                None => return Err(Error::UndefinedVariable(name, range)),
            };
            let rhs = compile_expression(expr, variables, functions)?;
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
                (_, r) => return Err(Error::TypeMismatchBinary(range, lhs.ty(), rhs.1, r.ty())),
            }
        }
        syntax::Statement::Declaration(range, name, expr) => {
            let rhs = compile_expression(expr, variables, functions)?;
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
                    let rc = Rc::new(RefCell::new("".to_string()));
                    variables.insert(name, value::Value::String(rc.clone()));
                    program::Statement::StringSubstitution(rc, expr)
                }
                program::Expression::Sound(expr) => {
                    let rc = Rc::new(RefCell::new(sound::Sound::Const(0.)));
                    variables.insert(name, value::Value::Sound(rc.clone()));
                    program::Statement::SoundSubstitution(rc, expr)
                }
                program::Expression::Void(_) => {
                    return Err(Error::VoidRHS(range));
                }
            }
        }
        syntax::Statement::Block(vec) => {
            let copied = &mut variables.clone();
            program::Statement::Block(
                vec.into_iter()
                    .map(|stmt| compile_statement(stmt, copied, functions))
                    .collect::<Result<_, _>>()?,
            )
        }
        syntax::Statement::While(expr, stmt) => {
            let cond = compile_expression(expr, variables, functions)?;
            let cond = match cond.0 {
                program::Expression::Boolean(expr) => expr,
                other => return Err(Error::TypeMismatchCond(cond.1, other.ty())),
            };
            let copied = &mut variables.clone();
            let stmt = compile_statement(*stmt, copied, functions)?;
            program::Statement::While(cond, stmt.into())
        }
        syntax::Statement::If(expr, stmt) => {
            let cond = compile_expression(expr, variables, functions)?;
            let cond = match cond.0 {
                program::Expression::Boolean(expr) => expr,
                other => return Err(Error::TypeMismatchCond(cond.1, other.ty())),
            };
            let copied = &mut variables.clone();
            let stmt = compile_statement(*stmt, copied, functions)?;
            program::Statement::If(cond, stmt.into())
        }
        _ => todo!(),
    })
}
