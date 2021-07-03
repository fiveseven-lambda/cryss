//! 型チェックを済ませたプログラム

use std::cell::Cell;
use std::rc::Rc;

use crate::sound;

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

pub enum BooleanExpression {
    Const(bool),
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

pub enum SoundExpression {
    Const(sound::Sound),
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

pub enum StringExpression {
    Const(String),
    Reference(RcCell<String>),
    Print(Box<StringExpression>),
    Add(Box<StringExpression>, Box<StringExpression>),
}

pub enum VoidExpression {}

pub enum Statement {
    Expression(Option<Expression>),
    RealSubstitution(RcCell<f64>, RealExpression),
    BooleanSubstitution(RcCell<bool>, BooleanExpression),
    SoundSubstitution(RcCell<sound::Sound>, SoundExpression),
    StringSubstitution(RcCell<String>, StringExpression),
}
