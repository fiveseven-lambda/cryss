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
    Real(Box<RealExpression>),
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
    Add(Box<StringExpression>),
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

pub enum FunctionStatement<T: TypeExpression> {
    Statement(Statement),
    Return(T),
}
