//! 型チェックを済ませた式木

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{sound, value};

type RcCell<T> = Rc<Cell<T>>;

pub enum Expression {
    Real(RealExpression),
    Boolean(BooleanExpression),
    Sound(SoundExpression),
    String(StringExpression),
    Void(VoidExpression),
    RealFunction(value::RealFunction),
    BooleanFunction(value::BooleanFunction),
    SoundFunction(value::SoundFunction),
    StringFunction(value::StringFunction),
    VoidFunction(value::VoidFunction),
}

/// デバッグ用：後で消す
impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Real(expr) => write!(f, "{:?}", expr),
            Expression::Boolean(expr) => write!(f, "{:?}", expr),
            Expression::Sound(expr) => write!(f, "{:?}", expr),
            Expression::String(_) => write!(f, "string"),
            Expression::Void(_) => write!(f, "void"),
            _ => write!(f, "function"),
        }
    }
}

#[derive(Debug)]
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

impl std::fmt::Debug for BooleanExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BooleanExpression::Const(_) => write!(f, "const"),
            BooleanExpression::Reference(_) => write!(f, "reference"),
            BooleanExpression::Print(expr) => write!(f, "({:?})?", expr),
            BooleanExpression::Not(expr) => write!(f, "-({:?})", expr),
            BooleanExpression::RealLess(left, right) => write!(f, "({:?}) < ({:?})", left, right),
            BooleanExpression::RealGreater(left, right) => {
                write!(f, "({:?}) > ({:?})", left, right)
            }
            BooleanExpression::RealEqual(left, right) => write!(f, "({:?}) == ({:?})", left, right),
            BooleanExpression::StringEqual(_, _) => write!(f, "string == string"),
            BooleanExpression::RealNotEqual(left, right) => {
                write!(f, "({:?}) != ({:?})", left, right)
            }
            BooleanExpression::StringNotEqual(_, _) => write!(f, "string != string"),
            BooleanExpression::And(left, right) => write!(f, "({:?}) && ({:?})", left, right),
            BooleanExpression::Or(left, right) => write!(f, "({:?}) || ({:?})", left, right),
        }
    }
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

impl std::fmt::Debug for SoundExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SoundExpression::Const(_) => write!(f, "const"),
            SoundExpression::Reference(_) => write!(f, "reference"),
            SoundExpression::Play(expr) => write!(f, "({:?})?", expr),
            SoundExpression::Real(expr) => write!(f, "Real({:?})", expr),
            SoundExpression::Minus(expr) => write!(f, "-({:?})", expr),
            SoundExpression::Reciprocal(expr) => write!(f, "/({:?})", expr),
            SoundExpression::Add(left, right) => write!(f, "({:?}) + ({:?})", left, right),
            SoundExpression::Sub(left, right) => write!(f, "({:?}) - ({:?})", left, right),
            SoundExpression::Mul(left, right) => write!(f, "({:?}) * ({:?})", left, right),
            SoundExpression::Div(left, right) => write!(f, "({:?}) / ({:?})", left, right),
            SoundExpression::Rem(left, right) => write!(f, "({:?}) % ({:?})", left, right),
            SoundExpression::Pow(left, right) => write!(f, "({:?}) ^ ({:?})", left, right),
            SoundExpression::LeftShift(left, right) => write!(f, "({:?}) << ({:?})", left, right),
            SoundExpression::RightShift(left, right) => write!(f, "({:?}) >> ({:?})", left, right),
        }
    }
}

pub enum StringExpression {
    Const(String),
    Reference(RcCell<String>),
    Print(Box<StringExpression>),
    Add(Box<StringExpression>, Box<StringExpression>),
}

pub enum VoidExpression {}

pub trait TypeExpression {
    type Output;
    fn evaluate(&self) -> Self::Output;
}
impl TypeExpression for RealExpression {
    type Output = f64;
    fn evaluate(&self) -> f64 {
        todo!();
    }
}
impl TypeExpression for BooleanExpression {
    type Output = bool;
    fn evaluate(&self) -> bool {
        todo!();
    }
}
impl TypeExpression for SoundExpression {
    type Output = sound::Sound;
    fn evaluate(&self) -> sound::Sound {
        todo!();
    }
}
impl TypeExpression for StringExpression {
    type Output = String;
    fn evaluate(&self) -> String {
        todo!();
    }
}
impl TypeExpression for VoidExpression {
    type Output = ();
    fn evaluate(&self) {
        todo!();
    }
}

// 関数の扱い方．
// まず，引数を格納する場所は RcCell<T> として持っておく．
// 関数の中身は文の並びとして持つ．
//
// ループが無いとき：各文を実行すると
// RealFunction なら `Some(f64)`
// BooleanFunction なら `Some(bool)`…
// が返る．

// 関数の呼び出し方．
// 中身は必ず Rc で包まれている
// 引数

pub enum Statement {
    Expression(Option<Expression>),
    RealSubstitution(RcCell<f64>, RealExpression),
    BooleanSubstitution(RcCell<bool>, BooleanExpression),
    SoundSubstitution(RcCell<sound::Sound>, SoundExpression),
    StringSubstitution(RcCell<String>, StringExpression),
}

impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Expression(expr) => {
                write!(f, "{:?}", expr)
            }
            _ => {
                write!(f, "other")
            }
        }
    }
}
pub enum FunctionStatement<T: TypeExpression> {
    Statement(Statement),
    Return(T),
}
