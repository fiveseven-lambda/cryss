//! 抽象構文木

use crate::pos;
use std::collections::HashMap;

/// None は空の式を表す
pub struct Expression(pub Option<(pos::Range, Node)>);
impl Expression {
    pub fn new(range: pos::Range, node: Node) -> Expression {
        Expression(Some((range, node)))
    }
    pub fn empty() -> Expression {
        Expression(None)
    }
    pub fn range(&self) -> Option<pos::Range> {
        self.0.as_ref().map(|(range, _)| range.clone())
    }
    pub fn try_into_identifier(self) -> Result<String, Expression> {
        match self.0 {
            Some((_, Node::Identifier(name))) => Ok(name),
            _ => Err(self),
        }
    }
}
/// デバッグ用　最後には消す
impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.0 {
            Some((_range, node)) => match f.alternate() {
                true => write!(f, "{:#?}", node),
                false => write!(f, "{:?}", node),
            },
            None => write!(f, "empty"),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Identifier(String),
    Parameter(String),
    Number(f64),
    String(String),
    Print(Box<Node>),
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
    Invocation(Box<Node>, Vec<Expression>, HashMap<String, Expression>),
    Lambda(Vec<Argument>, Box<Expression>),
    Group(Box<Expression>),
}

/// 関数定義における引数
#[derive(Debug)]
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
