//! 型チェックを済ませたプログラム

use std::cell::Cell;
use std::rc::Rc;

use crate::{error, pos, sound, types};

type RcCell<T> = Rc<Cell<T>>;

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
    Invocation(),
}

impl RealExpression {
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
            RealExpression::Reciprocal(expr) => 1. / expr.evaluate(),
            RealExpression::Add(left, right) => left.evaluate() + right.evaluate(),
            RealExpression::Sub(left, right) => left.evaluate() - right.evaluate(),
            RealExpression::Mul(left, right) => left.evaluate() * right.evaluate(),
            RealExpression::Div(left, right) => left.evaluate() / right.evaluate(),
            RealExpression::Rem(left, right) => left.evaluate() % right.evaluate(),
            RealExpression::Pow(left, right) => left.evaluate().powf(right.evaluate()),
            RealExpression::Invocation() => todo!(),
        }
    }
}

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
}

impl BooleanExpression {
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
        }
    }
}

pub enum SoundExpression {
    Reference(RcCell<sound::Sound>),
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
}

impl SoundExpression {
    fn evaluate(self) -> sound::Sound {
        todo!();
    }
}

pub enum StringExpression {
    Const(String),
    Reference(RcCell<String>),
    Print(Box<StringExpression>),
    Add(Box<StringExpression>, Box<StringExpression>),
}

impl StringExpression {
    fn evaluate(self) -> String {
        match self {
            StringExpression::Const(string) => string,
            StringExpression::Reference(_) => todo!(),
            StringExpression::Print(expr) => {
                let ret = expr.evaluate();
                println!("{}", ret);
                ret
            }
            StringExpression::Add(left, right) => left.evaluate() + &right.evaluate(),
        }
    }
}
pub enum VoidExpression {}
impl VoidExpression {
    fn evaluate(self) {
        todo!();
    }
}

pub enum Statement {
    Expression(Option<Expression>),
    RealSubstitution(RcCell<f64>, RealExpression),
    BooleanSubstitution(RcCell<bool>, BooleanExpression),
    SoundSubstitution(RcCell<sound::Sound>, SoundExpression),
    StringSubstitution(RcCell<String>, StringExpression),
}

impl Statement {
    pub fn run(self) {
        match self {
            Statement::Expression(expr) => {
                expr.map(|expr| expr.evaluate());
            }
            Statement::RealSubstitution(rc, expr) => {
                rc.set(expr.evaluate());
            }
            Statement::BooleanSubstitution(rc, expr) => {
                rc.set(expr.evaluate());
            }
            Statement::StringSubstitution(rc, expr) => {
                rc.set(expr.evaluate());
            }
            Statement::SoundSubstitution(rc, expr) => {
                rc.set(expr.evaluate());
            }
        }
    }
}
