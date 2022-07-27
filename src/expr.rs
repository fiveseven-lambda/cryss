#[derive(Debug)]
pub enum UnOp {
    Plus,
    Minus,
    Recip,
    LogicalNot,
    BitNot,
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    LeftShift,
    RightShift,
    ForwardShift,
    BackwardShift,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    BitAnd,
    BitOr,
    BitXor,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    ForwardShiftAssign,
    BackwardShiftAssign,
}

#[derive(Debug)]
pub enum PreExpr {
    Identifier(String),
    BinInt(String),
    OctInt(String),
    DecInt(String),
    HexInt(String),
    Float(String),
    String(String),
    UnOp(PUnOp, Box<PPreExpr>),
    BinOp(PBinOp, Box<PPreExpr>, Box<PPreExpr>),
    Group(Box<PPreExpr>),
    Call(Box<PPreExpr>, Vec<PPreExpr>),
}

impl From<PreExpr> for Expr {
    fn from(pre_expr: PreExpr) -> Expr {
        match pre_expr {
            PreExpr::Identifier(s) => Expr::Identifier(s),
            PreExpr::BinInt(s) => Expr::Integer(i32::from_str_radix(&s, 2).unwrap()),
            PreExpr::OctInt(s) => Expr::Integer(i32::from_str_radix(&s, 8).unwrap()),
            PreExpr::DecInt(s) => Expr::Integer(i32::from_str_radix(&s, 10).unwrap()),
            PreExpr::HexInt(s) => Expr::Integer(i32::from_str_radix(&s, 16).unwrap()),
            PreExpr::Float(s) => Expr::Float(s.parse().unwrap()),
            PreExpr::String(s) => Expr::String(s),
            PreExpr::UnOp((pos_op, op), operand) => {
                let op_name = match op {
                    UnOp::Minus => match operand.1 {
                        PreExpr::BinInt(s) => {
                            return Expr::Integer(i32::from_str_radix(&format!("-{s}"), 2).unwrap())
                        }
                        PreExpr::OctInt(s) => {
                            return Expr::Integer(i32::from_str_radix(&format!("-{s}"), 8).unwrap())
                        }
                        PreExpr::HexInt(s) => {
                            return Expr::Integer(
                                i32::from_str_radix(&format!("-{s}"), 10).unwrap(),
                            )
                        }
                        PreExpr::DecInt(s) => {
                            return Expr::Integer(format!("-{s}").parse().unwrap())
                        }
                        _ => "-",
                    },
                    UnOp::Plus => "+",
                    UnOp::Recip => "/",
                    UnOp::BitNot => "~",
                    UnOp::LogicalNot => "!",
                };
                let op = (pos_op, Expr::Identifier(op_name.to_string()));
                let operand = (operand.0, operand.1.into());
                Expr::Call(op.into(), vec![operand])
            }
            PreExpr::BinOp((pos_op, op), left, right) => {
                let op_name = match op {
                    BinOp::Add => "+",
                    BinOp::Sub => "-",
                    BinOp::Mul => "*",
                    BinOp::Div => "/",
                    BinOp::Rem => "%",
                    BinOp::LeftShift => "<<",
                    BinOp::RightShift => ">>",
                    BinOp::BackwardShift => "<<<",
                    BinOp::ForwardShift => ">>>",
                    BinOp::Equal => "==",
                    BinOp::NotEqual => "!=",
                    BinOp::Less => "<",
                    BinOp::Greater => ">",
                    BinOp::LessEqual => "<=",
                    BinOp::GreaterEqual => ">=",
                    BinOp::LogicalAnd => "&&",
                    BinOp::LogicalOr => "||",
                    BinOp::BitAnd => "&",
                    BinOp::BitOr => "|",
                    BinOp::BitXor => "^",
                    BinOp::Assign => "=",
                    BinOp::AddAssign => "+=",
                    BinOp::SubAssign => "-=",
                    BinOp::MulAssign => "*=",
                    BinOp::DivAssign => "/=",
                    BinOp::RemAssign => "%=",
                    BinOp::BitAndAssign => "&=",
                    BinOp::BitOrAssign => "|=",
                    BinOp::BitXorAssign => "^=",
                    BinOp::LeftShiftAssign => "<<=",
                    BinOp::RightShiftAssign => ">>=",
                    BinOp::BackwardShiftAssign => "<<<=",
                    BinOp::ForwardShiftAssign => ">>>=",
                };
                let op = (pos_op, Expr::Identifier(op_name.to_string()));
                let left = (left.0, left.1.into());
                let right = (right.0, right.1.into());
                Expr::Call(op.into(), vec![left, right])
            }
            PreExpr::Group(expr) => {
                let expr = (expr.0, expr.1.into());
                Expr::Group(expr.into())
            }
            PreExpr::Call(fnc, args) => {
                let fnc = (fnc.0, fnc.1.into());
                let args = args
                    .into_iter()
                    .map(|(pos, arg)| (pos, arg.into()))
                    .collect();
                Expr::Call(fnc.into(), args)
            }
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    Integer(i32),
    Float(f64),
    String(String),
    Group(Box<PExpr>),
    Call(Box<PExpr>, Vec<PExpr>),
}

use crate::pos;
pub type PBinOp = (pos::Range, BinOp);
pub type PUnOp = (pos::Range, UnOp);
pub type PPreExpr = (pos::Range, PreExpr);
pub type PExpr = (pos::Range, Expr);
