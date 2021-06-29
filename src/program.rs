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
    RealFunction(RealFunction),
    BooleanFunction(BooleanFunction),
    SoundFunction(SoundFunction),
    StringFunction(StringFunction),
    VoidFunction(VoidFunction),
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

pub enum RealFunctionStatement {
    RealExpression(RealExpression),
    RealSubstitution(RcCell<f64>, RealExpression),
    BooleanExpression(BooleanExpression),
    BooleanSubstitution(RcCell<bool>, BooleanExpression),
    SoundExpression(SoundExpression),
    SoundSubstitution(RcCell<sound::Sound>, SoundExpression),
    StringExpression(StringExpression),
    StringSubstitution(RcCell<String>, StringExpression),
    Return(RealExpression),
}

pub enum RealFunction {
    Const(value::RealFunction),
}
pub enum BooleanFunction {
    Const(value::BooleanFunction),
}
pub enum StringFunction {
    Const(value::StringFunction),
}
pub enum SoundFunction {
    Const(value::SoundFunction),
}
pub enum VoidFunction {
    Const(value::VoidFunction),
}
