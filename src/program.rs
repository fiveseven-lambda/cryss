//! 型チェックを済ませたプログラム

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::types;

use crate::function::{BooleanFunction, RealFunction, SoundFunction, StringFunction, VoidFunction};
use crate::pos;
use crate::sound::{self, Sound};

type RcCell<T> = Rc<Cell<T>>;
type RcRefCell<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub enum Expression {
    Real(RealExpression),
    Boolean(BooleanExpression),
    Sound(SoundExpression),
    String(StringExpression),
    Void(VoidExpression),
}

macro_rules! def_convert {
    ($from:ty => $to:ident::$name:ident) => {
        impl From<$from> for $to {
            fn from(expr: $from) -> $to {
                $to::$name(expr)
            }
        }
    };
}

def_convert!(RealExpression => Expression::Real);
def_convert!(BooleanExpression => Expression::Boolean);
def_convert!(SoundExpression => Expression::Sound);
def_convert!(StringExpression => Expression::String);
def_convert!(VoidExpression => Expression::Void);

pub trait Evaluatable: Sized {
    type Output;
    fn evaluate(self) -> Self::Output;

    /// いや，こいつには `Result<Self, Error>` を返させるべきかも
    fn from(_: Option<(Expression, pos::Range)>) -> Result<Self, Option<(Expression, pos::Range)>>;
}

impl Expression {
    pub fn ty(&self) -> types::Type {
        match self {
            Expression::Real(_) => types::Type::Real,
            Expression::Boolean(_) => types::Type::Boolean,
            Expression::Sound(_) => types::Type::Sound,
            Expression::String(_) => types::Type::String,
            Expression::Void(_) => types::Type::Void,
        }
    }
    pub fn evaluate(self) {
        match self {
            Expression::Real(expr) => {
                expr.evaluate();
            }
            Expression::Boolean(expr) => {
                expr.evaluate();
            }
            Expression::Sound(expr) => {
                expr.evaluate();
            }
            Expression::String(expr) => {
                expr.evaluate();
            }
            Expression::Void(expr) => {
                expr.evaluate();
            }
        }
    }
}

#[derive(Clone)]
pub enum Argument {
    Real(RcCell<f64>, RealExpression),
    Boolean(RcCell<bool>, BooleanExpression),
    Sound(RcRefCell<Sound>, SoundExpression),
    String(RcRefCell<String>, StringExpression),
}

impl Argument {
    fn set(self) {
        match self {
            Argument::Real(rc, expr) => rc.set(expr.evaluate()),
            Argument::Boolean(rc, expr) => rc.set(expr.evaluate()),
            Argument::Sound(rc, expr) => *rc.borrow_mut() = expr.evaluate(),
            Argument::String(rc, expr) => *rc.borrow_mut() = expr.evaluate(),
        }
    }
    fn evaluate(self) -> sound::Argument {
        match self {
            Argument::Real(rc, expr) => sound::Argument::Real(rc, expr.evaluate()),
            Argument::Boolean(rc, expr) => sound::Argument::Boolean(rc, expr.evaluate()),
            Argument::Sound(rc, expr) => sound::Argument::Sound(rc, expr.evaluate()),
            Argument::String(rc, expr) => sound::Argument::String(rc, expr.evaluate()),
        }
    }
}

#[derive(Clone)]
pub enum RealExpression {
    Const(f64),
    Reference(RcCell<f64>),
    Print(Box<RealExpression>),
    Minus(Box<RealExpression>),
    Reciprocal(Box<RealExpression>),
    Add(Box<RealExpression>, Box<RealExpression>),
    Sub(Box<RealExpression>, Box<RealExpression>),
    Mul(Box<RealExpression>, Box<RealExpression>),
    Div(Box<RealExpression>, Box<RealExpression>),
    Rem(Box<RealExpression>, Box<RealExpression>),
    Pow(Box<RealExpression>, Box<RealExpression>),
    Invocation(Rc<RealFunction>, Vec<Argument>),
}

impl Evaluatable for RealExpression {
    type Output = f64;
    fn evaluate(self) -> f64 {
        match self {
            RealExpression::Const(value) => value,
            RealExpression::Reference(rc) => rc.get(),
            RealExpression::Print(expr) => {
                let ret = expr.evaluate();
                println!("{}", ret);
                ret
            }
            RealExpression::Minus(expr) => -expr.evaluate(),
            RealExpression::Reciprocal(expr) => expr.evaluate().recip(),
            RealExpression::Add(left, right) => left.evaluate() + right.evaluate(),
            RealExpression::Sub(left, right) => left.evaluate() - right.evaluate(),
            RealExpression::Mul(left, right) => left.evaluate() * right.evaluate(),
            RealExpression::Div(left, right) => left.evaluate() / right.evaluate(),
            RealExpression::Rem(left, right) => left.evaluate() % right.evaluate(),
            RealExpression::Pow(left, right) => left.evaluate().powf(right.evaluate()),
            RealExpression::Invocation(fnc, arguments) => {
                arguments.into_iter().for_each(Argument::set);
                fnc.evaluate()
            }
        }
        .clamp(f64::MIN, f64::MAX)
    }
    fn from(
        expr: Option<(Expression, pos::Range)>,
    ) -> Result<RealExpression, Option<(Expression, pos::Range)>> {
        match expr {
            Some((Expression::Real(expr), _)) => Ok(expr),
            other => Err(other),
        }
    }
}

#[derive(Clone)]
pub enum BooleanExpression {
    Reference(RcCell<bool>),
    Print(Box<BooleanExpression>),
    Not(Box<BooleanExpression>),
    RealLess(Box<RealExpression>, Box<RealExpression>),
    RealGreater(Box<RealExpression>, Box<RealExpression>),
    RealEqual(Box<RealExpression>, Box<RealExpression>),
    StringEqual(Box<StringExpression>, Box<StringExpression>),
    RealNotEqual(Box<RealExpression>, Box<RealExpression>),
    StringNotEqual(Box<StringExpression>, Box<StringExpression>),
    And(Box<BooleanExpression>, Box<BooleanExpression>),
    Or(Box<BooleanExpression>, Box<BooleanExpression>),
    Invocation(Rc<BooleanFunction>, Vec<Argument>),
}

impl Evaluatable for BooleanExpression {
    type Output = bool;
    fn evaluate(self) -> bool {
        match self {
            BooleanExpression::Reference(rc) => rc.get(),
            BooleanExpression::Print(expr) => {
                let ret = expr.evaluate();
                println!("{}", ret);
                ret
            }
            BooleanExpression::Not(expr) => !expr.evaluate(),
            BooleanExpression::RealLess(left, right) => left.evaluate() < right.evaluate(),
            BooleanExpression::RealGreater(left, right) => left.evaluate() > right.evaluate(),
            BooleanExpression::RealEqual(left, right) => {
                (left.evaluate() - right.evaluate()).abs() <= 1e-6
            }
            BooleanExpression::RealNotEqual(left, right) => {
                (left.evaluate() - right.evaluate()).abs() > 1e-6
            }
            BooleanExpression::StringEqual(left, right) => left.evaluate() == right.evaluate(),
            BooleanExpression::StringNotEqual(left, right) => left.evaluate() != right.evaluate(),
            BooleanExpression::And(left, right) => left.evaluate() && right.evaluate(),
            BooleanExpression::Or(left, right) => left.evaluate() || right.evaluate(),
            BooleanExpression::Invocation(fnc, arguments) => {
                arguments.into_iter().for_each(Argument::set);
                fnc.evaluate()
            }
        }
    }
    fn from(
        expr: Option<(Expression, pos::Range)>,
    ) -> Result<BooleanExpression, Option<(Expression, pos::Range)>> {
        match expr {
            Some((Expression::Boolean(expr), _)) => Ok(expr),
            other => Err(other),
        }
    }
}

#[derive(Clone)]
pub enum SoundExpression {
    Reference(RcRefCell<Sound>),
    Play(Box<SoundExpression>),
    Real(RealExpression),
    Minus(Box<SoundExpression>),
    Reciprocal(Box<SoundExpression>),
    Add(Box<SoundExpression>, Box<SoundExpression>),
    Sub(Box<SoundExpression>, Box<SoundExpression>),
    Mul(Box<SoundExpression>, Box<SoundExpression>),
    Div(Box<SoundExpression>, Box<SoundExpression>),
    Rem(Box<SoundExpression>, Box<SoundExpression>),
    Pow(Box<SoundExpression>, Box<SoundExpression>),
    LeftShift(Box<SoundExpression>, Box<RealExpression>),
    RightShift(Box<SoundExpression>, Box<RealExpression>),
    Invocation(Rc<SoundFunction>, Vec<Argument>),
    Apply(
        Rc<RealFunction>,
        Vec<Argument>,
        Vec<(RcCell<f64>, SoundExpression)>,
    ),
}

impl Evaluatable for SoundExpression {
    type Output = Sound;
    fn evaluate(self) -> Sound {
        match self {
            SoundExpression::Reference(rc) => rc.borrow().clone(),
            SoundExpression::Invocation(fnc, arguments) => {
                arguments.into_iter().for_each(Argument::set);
                fnc.evaluate()
            }
            SoundExpression::Real(expr) => Sound::Const(expr.evaluate()),
            SoundExpression::Apply(fnc, arguments, sounds) => Sound::Apply(
                fnc,
                arguments.into_iter().map(Argument::evaluate).collect(),
                sounds
                    .into_iter()
                    .map(|(rc, expr)| (rc, expr.evaluate()))
                    .collect(),
            ),
            SoundExpression::Play(expr) => expr.evaluate(),
            SoundExpression::Minus(expr) => Sound::Minus(expr.evaluate().into()),
            SoundExpression::Reciprocal(expr) => Sound::Reciprocal(expr.evaluate().into()),
            SoundExpression::Add(left, right) => {
                Sound::Add(left.evaluate().into(), right.evaluate().into())
            }
            SoundExpression::Sub(left, right) => {
                Sound::Sub(left.evaluate().into(), right.evaluate().into())
            }
            SoundExpression::Mul(left, right) => {
                Sound::Mul(left.evaluate().into(), right.evaluate().into())
            }
            SoundExpression::Div(left, right) => {
                Sound::Div(left.evaluate().into(), right.evaluate().into())
            }
            SoundExpression::Rem(left, right) => {
                Sound::Rem(left.evaluate().into(), right.evaluate().into())
            }
            SoundExpression::Pow(left, right) => {
                Sound::Pow(left.evaluate().into(), right.evaluate().into())
            }
            SoundExpression::LeftShift(left, right) => left.evaluate().shift(right.evaluate()),
            SoundExpression::RightShift(left, right) => left.evaluate().shift(-right.evaluate()),
        }
    }
    fn from(
        expr: Option<(Expression, pos::Range)>,
    ) -> Result<SoundExpression, Option<(Expression, pos::Range)>> {
        match expr {
            Some((Expression::Sound(expr), _)) => Ok(expr),
            Some((Expression::Real(expr), _)) => Ok(SoundExpression::Real(expr)),
            other => Err(other),
        }
    }
}

#[derive(Clone)]
pub enum StringExpression {
    Const(String),
    Reference(RcRefCell<String>),
    Print(Box<StringExpression>),
    Add(Box<StringExpression>, Box<StringExpression>),
    Invocation(Rc<StringFunction>, Vec<Argument>),
}

impl Evaluatable for StringExpression {
    type Output = String;
    fn evaluate(self) -> String {
        match self {
            StringExpression::Const(string) => string,
            StringExpression::Reference(rc) => rc.borrow().clone(),
            StringExpression::Print(expr) => {
                let ret = expr.evaluate();
                println!("{}", ret);
                ret
            }
            StringExpression::Add(left, right) => left.evaluate() + &right.evaluate(),
            StringExpression::Invocation(fnc, arguments) => {
                arguments.into_iter().for_each(Argument::set);
                fnc.evaluate()
            }
        }
    }
    fn from(
        expr: Option<(Expression, pos::Range)>,
    ) -> Result<StringExpression, Option<(Expression, pos::Range)>> {
        match expr {
            Some((Expression::String(expr), _)) => Ok(expr),
            other => Err(other),
        }
    }
}
#[derive(Clone)]
pub enum VoidExpression {
    Const,
    Invocation(Rc<VoidFunction>, Vec<Argument>),
}
impl Evaluatable for VoidExpression {
    type Output = ();
    fn evaluate(self) {
        match self {
            VoidExpression::Const => (/* do nothing */),
            VoidExpression::Invocation(fnc, arguments) => {
                arguments.into_iter().for_each(Argument::set);
                fnc.evaluate()
            }
        }
    }
    fn from(
        expr: Option<(Expression, pos::Range)>,
    ) -> Result<VoidExpression, Option<(Expression, pos::Range)>> {
        match expr {
            None => Ok(VoidExpression::Const),
            other => Err(other),
        }
    }
}

#[derive(Clone)]
pub enum Statement<Expr: Evaluatable> {
    Expression(Option<Expression>),
    RealSubstitution(RcCell<f64>, RealExpression),
    BooleanSubstitution(RcCell<bool>, BooleanExpression),
    SoundSubstitution(RcRefCell<Sound>, SoundExpression),
    StringSubstitution(RcRefCell<String>, StringExpression),
    While(BooleanExpression, Box<Statement<Expr>>),
    If(
        BooleanExpression,
        Box<Statement<Expr>>,
        Box<Option<Statement<Expr>>>,
    ),
    Block(Vec<Statement<Expr>>),
    Return(Expr),
}

impl<Expr: Evaluatable + Clone> Statement<Expr> {
    pub fn run(self) -> Option<Expr::Output> {
        match self {
            Statement::Expression(expr) => {
                if let Some(expr) = expr {
                    Expression::evaluate(expr)
                }
                None
            }
            Statement::RealSubstitution(rc, expr) => {
                rc.set(expr.evaluate());
                None
            }
            Statement::BooleanSubstitution(rc, expr) => {
                rc.set(expr.evaluate());
                None
            }
            Statement::StringSubstitution(rc, expr) => {
                *rc.borrow_mut() = expr.evaluate();
                None
            }
            Statement::SoundSubstitution(rc, expr) => {
                *rc.borrow_mut() = expr.evaluate();
                None
            }
            Statement::While(cond, stmt) => {
                while cond.clone().evaluate() {
                    if let Some(value) = stmt.clone().run() {
                        return Some(value);
                    }
                }
                None
            }
            Statement::If(cond, stmt1, stmt2) => {
                if cond.evaluate() {
                    stmt1.run()
                } else {
                    stmt2.map(Statement::run).flatten()
                }
            }
            Statement::Block(vec) => {
                for stmt in vec {
                    if let Some(value) = stmt.run() {
                        return Some(value);
                    }
                }
                None
            }
            Statement::Return(expr) => Some(expr.evaluate()),
        }
    }
}
