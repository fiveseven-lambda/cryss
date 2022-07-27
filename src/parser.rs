use crate::error::Error;
use crate::expr;
use crate::lexer::Lexer;
use crate::token::Token;

fn parse_factor(lexer: &mut Lexer) -> Result<Option<expr::PPreExpr>, Error> {
    let pos;
    let expr;
    if let Some((pos_token, token)) = lexer.next_if(|token| {
        matches!(
            token,
            Token::Identifier(_)
                | Token::BinInt(_)
                | Token::DecInt(_)
                | Token::OctInt(_)
                | Token::HexInt(_)
                | Token::Float(_)
                | Token::String(_)
        )
    })? {
        pos = pos_token;
        expr = match token {
            Token::Identifier(s) => expr::PreExpr::Identifier(s),
            Token::BinInt(s) => expr::PreExpr::BinInt(s),
            Token::OctInt(s) => expr::PreExpr::OctInt(s),
            Token::DecInt(s) => expr::PreExpr::DecInt(s),
            Token::HexInt(s) => expr::PreExpr::HexInt(s),
            Token::Float(s) => expr::PreExpr::Float(s),
            Token::String(s) => expr::PreExpr::String(s),
            _ => unreachable!(),
        };
    } else if let Some(op) = lexer.next_if_map(|token| match token {
        Token::Plus => Some(expr::UnOp::Plus),
        Token::Hyphen => Some(expr::UnOp::Minus),
        Token::Slash => Some(expr::UnOp::Recip),
        Token::Exclamation => Some(expr::UnOp::LogicalNot),
        Token::Tilde => Some(expr::UnOp::BitNot),
        _ => None,
    })? {
        match parse_factor(lexer)? {
            Some(operand) => {
                pos = &op.0 + &operand.0;
                expr = expr::PreExpr::UnOp(op, operand.into());
            }
            None => panic!(),
        }
    } else if let Some((pos_open, _)) =
        lexer.next_if(|token| matches!(token, Token::OpeningParenthesis))?
    {
        let inner = parse_expr(lexer)?;
        let pos_close = match lexer.next()? {
            Some((pos_close, Token::ClosingParenthesis)) => pos_close,
            _ => panic!(),
        };
        expr = match inner {
            Some(inner) => expr::PreExpr::Group(inner.into()),
            None => panic!(),
        };
        pos = pos_open + pos_close;
    } else {
        return Ok(None);
    };
    let mut ret = (pos, expr);
    loop {
        if let Some(..) = lexer.next_if(|token| matches!(token, Token::OpeningParenthesis))? {
            let args = parse_list(lexer)?;
            let pos_close = match lexer.next()? {
                Some((pos_close, Token::ClosingParenthesis)) => pos_close,
                _ => panic!(),
            };
            let pos = &ret.0 + pos_close;
            let expr = expr::PreExpr::Call(ret.into(), args);
            ret = (pos, expr);
        } else {
            return Ok(Some(ret));
        }
    }
}

use enum_iterator::Sequence;
#[derive(Clone, Copy, Sequence, PartialEq, Eq, Debug)]
enum Precedence {
    Assign,
    TimeShift,
    LogicalOr,
    LogicalAnd,
    Comparison,
    BitOr,
    BitXor,
    BitAnd,
    BitShift,
    AddSub,
    MulDivRem,
    Max,
}

impl expr::BinOp {
    fn precedence(&self) -> Precedence {
        match self {
            Self::Mul | Self::Div | Self::Rem => Precedence::MulDivRem,
            Self::Add | Self::Sub => Precedence::AddSub,
            Self::LeftShift | Self::RightShift => Precedence::BitShift,
            Self::BitAnd => Precedence::BitAnd,
            Self::BitXor => Precedence::BitXor,
            Self::BitOr => Precedence::BitOr,
            Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual => Precedence::Comparison,
            Self::LogicalAnd => Precedence::LogicalAnd,
            Self::LogicalOr => Precedence::LogicalOr,
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::RemAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::BitAndAssign
            | Self::BitOrAssign
            | Self::BitXorAssign
            | Self::ForwardShiftAssign
            | Self::BackwardShiftAssign => Precedence::Assign,
            Self::ForwardShift | Self::BackwardShift => Precedence::TimeShift,
        }
    }
}

enum Assoc {
    LeftToRight,
    RightToLeft,
}
impl Precedence {
    fn assoc(&self) -> Assoc {
        match self {
            Precedence::Assign => Assoc::RightToLeft,
            _ => Assoc::LeftToRight,
        }
    }
}

fn parse_bin_op(lexer: &mut Lexer, prec: Precedence) -> Result<Option<expr::PPreExpr>, Error> {
    let nprec = match prec.next() {
        Some(nprec) => nprec,
        None => return parse_factor(lexer),
    };
    let mut ret = match parse_bin_op(lexer, nprec)? {
        Some(expr) => expr,
        None => return Ok(None),
    };
    loop {
        if let Some(op) = lexer.next_if_map(|token| {
            let op = match token {
                Token::Plus => expr::BinOp::Add,
                Token::PlusEqual => expr::BinOp::AddAssign,
                Token::Hyphen => expr::BinOp::Sub,
                Token::HyphenEqual => expr::BinOp::SubAssign,
                Token::Asterisk => expr::BinOp::Mul,
                Token::AsteriskEqual => expr::BinOp::MulAssign,
                Token::Slash => expr::BinOp::Div,
                Token::SlashEqual => expr::BinOp::DivAssign,
                Token::Percent => expr::BinOp::Rem,
                Token::PercentEqual => expr::BinOp::RemAssign,
                Token::Equal => expr::BinOp::Assign,
                Token::DoubleEqual => expr::BinOp::Equal,
                Token::ExclamationEqual => expr::BinOp::NotEqual,
                Token::Less => expr::BinOp::Less,
                Token::Greater => expr::BinOp::Greater,
                Token::LessEqual => expr::BinOp::LessEqual,
                Token::GreaterEqual => expr::BinOp::GreaterEqual,
                Token::DoubleLess => expr::BinOp::LeftShift,
                Token::DoubleLessEqual => expr::BinOp::LeftShiftAssign,
                Token::DoubleGreater => expr::BinOp::RightShift,
                Token::DoubleGreaterEqual => expr::BinOp::RightShiftAssign,
                Token::TripleLess => expr::BinOp::BackwardShift,
                Token::TripleLessEqual => expr::BinOp::BackwardShiftAssign,
                Token::TripleGreater => expr::BinOp::ForwardShift,
                Token::TripleGreaterEqual => expr::BinOp::ForwardShiftAssign,
                Token::Ampersand => expr::BinOp::BitAnd,
                Token::AmpersandEqual => expr::BinOp::BitAndAssign,
                Token::DoubleAmpersand => expr::BinOp::LogicalAnd,
                Token::Bar => expr::BinOp::BitOr,
                Token::BarEqual => expr::BinOp::BitOrAssign,
                Token::DoubleBar => expr::BinOp::LogicalOr,
                Token::Circumflex => expr::BinOp::BitXor,
                Token::CircumflexEqual => expr::BinOp::BitXorAssign,
                _ => return None,
            };
            (op.precedence() == prec).then(|| op)
        })? {
            let assoc = prec.assoc();
            let right = match parse_bin_op(
                lexer,
                match assoc {
                    Assoc::LeftToRight => nprec,
                    Assoc::RightToLeft => prec,
                },
            )? {
                Some(expr) => expr,
                None => panic!(),
            };
            let pos = &ret.0 + &right.0;
            let expr = expr::PreExpr::BinOp(op, ret.into(), right.into());
            ret = (pos, expr);
            if matches!(assoc, Assoc::LeftToRight) {
                continue;
            }
        }
        return Ok(Some(ret));
    }
}

pub fn parse_expr(lexer: &mut Lexer) -> Result<Option<expr::PPreExpr>, Error> {
    parse_bin_op(lexer, Precedence::first().unwrap())
}

pub fn parse_list(lexer: &mut Lexer) -> Result<Vec<expr::PPreExpr>, Error> {
    let mut ret = Vec::new();
    loop {
        let elem = parse_expr(lexer)?;
        if let Some(_) = lexer.next_if(|token| matches!(token, Token::Comma))? {
            match elem {
                Some(elem) => ret.push(elem),
                None => panic!(),
            }
        } else {
            if let Some(elem) = elem {
                ret.push(elem);
            }
            return Ok(ret);
        }
    }
}
