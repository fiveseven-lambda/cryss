//! トークンを抽象構文木に変換する．

use crate::{error, lexer, pos, syntax, token};
use syntax::{Expression, Node};
use token::Token;

use std::collections::HashMap;
use std::io::BufRead;

/// パースした式と，その直後のトークン
type Result<T> = std::result::Result<(T, Option<(pos::Range, Token)>), error::Error>;

pub fn parse_factor(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<Expression> {
    let (mut range, mut node) = match lexer.next(log)? {
        Some((range, Token::Identifier(name))) => (range, Node::Identifier(name)),
        Some((range, Token::Parameter(name))) => (range, Node::Parameter(name)),
        Some((range, Token::Number(value))) => (range, Node::Number(value)),
        Some((range, Token::String(string))) => (range, Node::String(string)),
        Some((range, Token::Minus)) => {
            // 単項マイナス演算子
            let mut ret = parse_factor(lexer, log)?;
            ret.0 = Expression::new(range + ret.0.range(), Node::Minus(ret.0.into()));
            return Ok(ret);
        }
        Some((range, Token::Slash)) => {
            // 単項逆数演算子
            let mut ret = parse_factor(lexer, log)?;
            ret.0 = Expression::new(range + ret.0.range(), Node::Reciprocal(ret.0.into()));
            return Ok(ret);
        }
        Some((range, Token::Exclamation)) => {
            // 単項否定演算子
            let mut ret = parse_factor(lexer, log)?;
            ret.0 = Expression::new(range + ret.0.range(), Node::Not(ret.0.into()));
            return Ok(ret);
        }
        Some((open, Token::OpeningParen)) => {
            // 括弧でくくられた部分
            match parse_operator(lexer, log)? {
                (expression, Some((close, Token::ClosingParen))) => {
                    (open + close, Node::Group(expression.into()))
                }
                (_, other) => {
                    let range = other.map(|(range, _)| range);
                    return Err(error::Error::UnclosedParenUntil(open, range));
                }
            }
        }
        other => return Ok((Expression::empty(), other)),
    };
    loop {
        match lexer.next(log)? {
            Some((open, Token::OpeningParen)) => {
                // 関数呼び出し
                let ((vec, map), end) = parse_arguments(lexer, log)?;
                match end {
                    Some((close, Token::ClosingParen)) => {
                        range = range + close;
                        node = Node::Invocation(node.into(), vec, map);
                    }
                    other => {
                        let range = other.map(|(range, _)| range);
                        return Err(error::Error::UnclosedParenUntil(open, range));
                    }
                }
            }
            Some((question, Token::Question)) => {
                range = range + question;
                node = Node::Print(node.into());
            }
            other => return Ok((Expression::new(range, node), other)),
        }
    }
}

macro_rules! def_binary_operator {
    ($prev:ident => $next:ident: $($from:path => $to:expr),* $(,)?) => {
        pub fn $next(
            lexer: &mut lexer::Lexer<impl BufRead>,
            log: &mut Vec<String>,
        ) -> Result<Expression> {
            let mut ret = $prev(lexer, log)?;
            loop {
                match ret {
                    $((left, Some((range, $from))) => {
                        ret = $prev(lexer, log)?;
                        ret.0 = Expression::new(
                            left.range() + range + ret.0.range(),
                            $to(left.into(), ret.0.into())
                        );
                    }),*
                    _ => return Ok(ret),
                }
            }
        }
    };
}

def_binary_operator! {
    parse_factor => parse_operator1:
        Token::DoubleLess => Node::LeftShift,
        Token::DoubleGreater => Node::RightShift,
}
def_binary_operator! {
    parse_operator1 => parse_operator2:
        Token::Circumflex => Node::Pow,
}
def_binary_operator! {
    parse_operator2 => parse_operator3:
        Token::Asterisk => Node::Mul,
        Token::Slash => Node::Div,
        Token::Percent => Node::Rem,
}
def_binary_operator! {
    parse_operator3 => parse_operator4:
        Token::Plus => Node::Add,
        Token::Minus => Node::Sub,
}
def_binary_operator! {
    parse_operator4 => parse_operator5:
        Token::Less => Node::Less,
        Token::Greater => Node::Greater,
}
def_binary_operator! {
    parse_operator5 => parse_operator6:
        Token::DoubleEqual => Node::Equal,
        Token::ExclamationEqual => Node::NotEqual,
}
def_binary_operator! {
    parse_operator6 => parse_operator:
        Token::DoubleAmpersand => Node::And,
        Token::DoubleBar => Node::Or,
}

pub fn parse_arguments(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<(Vec<Expression>, HashMap<String, Expression>)> {
    let mut vec = Vec::new();
    let mut map = HashMap::new();
    loop {
        let (item, end) = parse_operator(lexer, log)?;
        match end {
            Some((_, Token::Comma)) => vec.push(item),
            Some((equal, Token::Equal)) => {
                let name = match item.try_into_identifier() {
                    Ok(name) => name,
                    Err(item) => {
                        return Err(error::Error::ArgumentNameNotIdentifier(item.range(), equal))
                    }
                };
                let (item, end) = parse_operator(lexer, log)?;
                map.insert(name, item);
                if !matches!(end, Some((_, Token::Comma))) {
                    return Ok(((vec, map), end));
                }
            }
            _ => {
                vec.push(item);
                return Ok(((vec, map), end));
            }
        }
    }
}

pub fn parse_expression(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<Expression> {
    parse_operator(lexer, log)
}
