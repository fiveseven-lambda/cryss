//! 抽象構文木

use crate::pos;
use std::collections::HashMap;

/// 式
pub struct Expr {
    pub range: pos::Range,
    pub node: ExprNode,
}

impl Expr {
    pub fn new(range: pos::Range, node: ExprNode) -> Expr {
        Expr { range, node }
    }
}

/// 式を表す木のノード
#[derive(Debug)]
pub enum ExprNode {
    /// 識別子
    Identifier(String),
    /// 関数呼び出し
    Invocation(String, Vec<Expr>, HashMap<String, Expr>),
    /// 属性（先頭が `$` で始まるもの）
    Parameter(String),
    /// 数値リテラル
    Number(f64),
    /// 文字列リテラル
    String(String),
    /// 出力（後置演算子 `?` ）
    Print(Box<Expr>),
    /// 負号（前置演算子 `-` ）
    Minus(Box<Expr>),
    /// 逆数（前置演算子 `/` ）
    Reciprocal(Box<Expr>),
    /// 否定（前置演算子 `!` ）
    Not(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    LeftShift(Box<Expr>, Box<Expr>),
    RightShift(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    /// 丸括弧 `( )` でくくられた部分
    Group(Box<Expr>),
    /// 角括弧 `[ ]` でくくって `,` `;` で区切る
    Score(Vec<Vec<Expr>>),
}

/// 文
pub struct Stmt {
    range: pos::Range,
    node: StmtNode,
}

impl Stmt {
    fn new(range: pos::Range, node: StmtNode) -> Stmt {
        Stmt { range, node }
    }
}

pub enum StmtNode {
    /// 式だけの文
    Expression(Option<Expr>),
    /// 代入文
    Substitution(String, Expr),
    /// 宣言と代入
    Declaration(String, Expr),
    /// 波括弧 `{ }` で囲まれたブロック
    Block(Vec<Stmt>),
    /// if 文
    If(Expr, Box<Stmt>),
    /// while 文
    While(Expr, Box<Stmt>),
    Break,
    Continue,
}

use std::fmt::{Debug, Formatter, Result as FResult};
/// デバッグ用！あとで消す
impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> FResult {
        write!(f, "{:#?} ({})", self.node, self.range)
    }
}
