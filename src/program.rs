//! 型チェックを済ませた式木

use std::cell::Cell;
use std::rc::Rc;

use crate::sound;

type RcCell<T> = Rc<Cell<T>>;

enum RealExpression {
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
}

enum BooleanExpression {
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

enum SoundExpression {
    Const(sound::Sound),
    Reference(RcCell<sound::Sound>),
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

enum StringExpression {
    Const(String),
    Reference(RcCell<StringExpression>),
    Print(Box<StringExpression>),
    Add(Box<StringExpression>),
}

enum VoidExpression {}

enum Statement {
    RealExpression(Box<RealExpression>),
    RealSubstitution(RcCell<f64>, Box<RealExpression>),
    RealDeclaration(String, Box<RealExpression>),
    BooleanExpression(Box<BooleanExpression>),
    BooleanSubstitution(RcCell<bool>, Box<BooleanExpression>),
    BooleanDeclaration(String, Box<BooleanExpression>),
    SoundExpression(Box<SoundExpression>),
    SoundSubstitution(RcCell<sound::Sound>, Box<SoundExpression>),
    SoundDeclaration(String, Box<SoundExpression>),
    StringExpression(Box<StringExpression>),
    StringSubstitution(RcCell<String>, Box<StringExpression>),
    StringDeclaration(String, Box<StringExpression>),
    If(Box<BooleanExpression>, Vec<Statement>),
    While(Box<BooleanExpression>, Vec<LoopStatement>),
}

enum LoopStatement {
    RealExpression(Box<RealExpression>),
    RealSubstitution(RcCell<f64>, Box<RealExpression>),
    RealDeclaration(String, Box<RealExpression>),
    BooleanExpression(Box<BooleanExpression>),
    BooleanSubstitution(RcCell<bool>, Box<BooleanExpression>),
    BooleanDeclaration(String, Box<BooleanExpression>),
    SoundExpression(Box<SoundExpression>),
    SoundSubstitution(RcCell<sound::Sound>, Box<SoundExpression>),
    SoundDeclaration(String, Box<SoundExpression>),
    StringExpression(Box<StringExpression>),
    StringSubstitution(RcCell<String>, Box<StringExpression>),
    StringDeclaration(String, Box<StringExpression>),
    If(Box<BooleanExpression>, Vec<LoopStatement>),
    While(Box<BooleanExpression>, Vec<LoopStatement>),
    Break,
    Continue,
}
