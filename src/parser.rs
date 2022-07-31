use crate::error::Error;
use crate::expr;
use crate::lexer::Lexer;
use crate::sentence;
use crate::token::Token;

pub fn parse_sentence(lexer: &mut Lexer) -> Result<Option<sentence::PPreSentence>, Error> {
    match (parse_expr(lexer)?, lexer.next()?) {
        (expr, Some((pos_semicolon, Token::Semicolon))) => {
            let pos = match &expr {
                Some((pos, _)) => pos + pos_semicolon,
                None => pos_semicolon,
            };
            Ok(Some((pos, sentence::PreSentence::Expr(expr))))
        }
        (None, None) => Ok(None),
        _ => panic!(),
    }
}

fn parse_expr(lexer: &mut Lexer) -> Result<Option<expr::PPreExpr>, Error> {
    parse_bin_op(lexer, Precedence::first().unwrap())
}

fn parse_factor(lexer: &mut Lexer) -> Result<Option<expr::PPreExpr>, Error> {
    let mut ret = if let Some((pos, Token::Identifier(s))) = lexer.next_if(Token::is_identifier)? {
        (pos, expr::PreExpr::Identifier(s))
    } else if let Some((pos, Token::BinInt(s))) = lexer.next_if(Token::is_bin_int)? {
        (pos, expr::PreExpr::BinInt(s))
    } else if let Some((pos, Token::OctInt(s))) = lexer.next_if(Token::is_oct_int)? {
        (pos, expr::PreExpr::OctInt(s))
    } else if let Some((pos, Token::DecInt(s))) = lexer.next_if(Token::is_dec_int)? {
        (pos, expr::PreExpr::DecInt(s))
    } else if let Some((pos, Token::HexInt(s))) = lexer.next_if(Token::is_hex_int)? {
        (pos, expr::PreExpr::HexInt(s))
    } else if let Some((pos, Token::Float(s))) = lexer.next_if(Token::is_float)? {
        (pos, expr::PreExpr::Float(s))
    } else if let Some((pos, Token::String(s))) = lexer.next_if(Token::is_string)? {
        (pos, expr::PreExpr::String(s))
    } else if let Some(op) = lexer.next_if_map(|token| match token {
        Token::Plus => Some(expr::UnOp::Plus),
        Token::Hyphen => Some(expr::UnOp::Minus),
        Token::Slash => Some(expr::UnOp::Recip),
        Token::Exclamation => Some(expr::UnOp::LogicalNot),
        Token::Tilde => Some(expr::UnOp::BitNot),
        _ => None,
    })? {
        match parse_factor(lexer)? {
            Some(operand) => (&op.0 + &operand.0, expr::PreExpr::UnOp(op, operand.into())),
            None => {
                return Err(match lexer.next()? {
                    Some((pos, _)) => Error::UnexpectedTokenAfterPrefixOperator(op.0, pos),
                    None => Error::UnexpectedEOFAfterPrefixOperator(op.0),
                })
            }
        }
    } else if let Some((pos_open, _)) = lexer.next_if(Token::is_opening_parenthesis)? {
        let inner = parse_expr(lexer)?;
        let pos_close = match lexer.next()? {
            Some((pos_close, Token::ClosingParenthesis)) => pos_close,
            Some((pos, _)) => return Err(Error::UnexpectedTokenInParenthesis(pos_open, pos)),
            None => return Err(Error::NoClosingParenthesis(pos_open)),
        };
        let expr = match inner {
            Some(inner) => expr::PreExpr::Group(inner.into()),
            None => return Err(Error::EmptyParenthesis(pos_open, pos_close)),
        };
        let pos = pos_open + pos_close;
        (pos, expr)
    } else {
        return Ok(None);
    };
    loop {
        if let Some((pos_open, _)) = lexer.next_if(Token::is_opening_parenthesis)? {
            let args = parse_list(lexer)?;
            let pos_close = match lexer.next()? {
                Some((pos_close, Token::ClosingParenthesis)) => pos_close,
                Some((pos, _)) => return Err(Error::UnexpectedTokenInParenthesis(pos_open, pos)),
                None => return Err(Error::NoClosingParenthesis(pos_open)),
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
#[derive(Clone, Copy, Sequence, PartialEq, Eq)]
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
                None => {
                    return Err(match lexer.next()? {
                        Some((pos, _)) => Error::UnexpectedTokenAfterBinaryOperator(op.0, pos),
                        None => Error::UnexpectedEOFAfterBinaryOperator(op.0),
                    })
                }
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

fn parse_list(lexer: &mut Lexer) -> Result<Vec<expr::PPreExpr>, Error> {
    let mut ret = Vec::new();
    loop {
        let elem = parse_expr(lexer)?;
        if let Some((pos_comma, _)) = lexer.next_if(Token::is_comma)? {
            match elem {
                Some(elem) => ret.push(elem),
                None => return Err(Error::NoExpressionBeforeComma(pos_comma)),
            }
        } else {
            if let Some(elem) = elem {
                ret.push(elem);
            }
            return Ok(ret);
        }
    }
}
