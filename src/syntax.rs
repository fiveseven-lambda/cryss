//! 抽象構文木

use crate::pos;
use std::collections::HashMap;

/// 式
pub struct Expression {
    pub range: pos::Range,
    pub node: Node,
}

impl Expression {
    pub fn new(range: pos::Range, node: Node) -> Expression {
        Expression { range, node }
    }
}

/// 式を表す木のノード
#[derive(Debug)]
pub enum Node {
    /// 識別子
    Identifier(String),
    /// 関数呼び出し
    Invocation(String, Vec<Expression>, HashMap<String, Expression>),
    /// 属性（先頭が `$` で始まるもの）
    Parameter(String),
    /// 数値リテラル
    Number(f64),
    /// 文字列リテラル
    String(String),
    /// 出力（後置演算子 `?` ）
    Print(Box<Expression>),
    /// 負号（前置演算子 `-` ）
    Minus(Box<Expression>),
    /// 逆数（前置演算子 `/` ）
    Reciprocal(Box<Expression>),
    /// 否定（前置演算子 `!` ）
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
    /// 丸括弧 `( )` でくくられた部分
    Group(Box<Expression>),
    /// 角括弧 `[ ]` でくくって `,` `;` で区切る
    Score(Vec<Vec<Expression>>),
}

/// 文
#[derive(Debug)]
pub enum Statement {
    /// 式だけの文
    Expression(Option<Expression>),
    /// 代入文
    Substitution(pos::Range, String, Expression),
    /// 宣言と代入
    Declaration(pos::Range, String, Expression),
    /// 波括弧 `{ }` で囲まれたブロック
    Block(Vec<Statement>),
    /// if 文
    If(Expression, Box<Statement>, Box<Option<Statement>>),
    /// while 文
    While(Expression, Box<Statement>),
    Break,
    Continue,
}

use std::fmt::{Debug, Formatter, Result as FResult};
/// デバッグ用！あとで消す
impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter) -> FResult {
        write!(f, "{:#?} ({})", self.node, self.range)
    }
}
