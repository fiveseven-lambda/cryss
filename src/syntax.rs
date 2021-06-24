//! 抽象構文木

use crate::pos;
use std::collections::HashMap;

/// None は空の式を表す
type Expression = Option<(pos::Range, Node)>;

pub enum Node {
    Identifier(String),
    Parameter(String),
    Number(f64),
    String(String),
    Print(Box<Expression>), // 後置 ? 演算子
    Minus(Box<Expression>),
    Reciprocal(Box<Expression>),
    Not(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Rem(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
    LeftShift(Box<Expression>, Box<Expression>),
    RightShift(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Invocation(
        Box<Expression>,
        Vec<Expression>,
        HashMap<String, Expression>,
    ),
    Lambda(Vec<Argument>, Box<Expression>),
    Group(Box<Expression>),
}

/// 関数定義における引数
pub enum Argument {
    Real(String, Option<Expression>),
    Boolean(String, Option<Expression>),
    Sound(String, Option<Expression>),
    String(String, Option<Expression>),
}

pub enum Statement {
    Expression(Expression),
    Substitution(String, Expression),
    Declaration(String, Expression),
    While(Expression, Vec<Statement>),
    If(Expression, Vec<Statement>),
    Break,
    Continue,
}
