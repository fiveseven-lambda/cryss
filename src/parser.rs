//! トークンを抽象構文木に変換する．

use crate::{error, lexer, pos, syntax, token};
use error::Error;
use syntax::{Expr, ExprNode, Stmt};
use token::Token;

use std::collections::HashMap;
use std::io::BufRead;

/// パースしたものと，その直後のトークン
type Parsed<T> = (T, Option<(pos::Range, Token)>);

fn parse_factor(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<Parsed<Option<Expr>>, Error> {
    let expression = match lexer.next(log)? {
        Some((range, Token::Identifier(name))) => match lexer.next(log)? {
            // 関数呼び出し
            Some((open, Token::OpeningParenthesis)) => {
                let ((vec, map), end) = parse_invocation_arguments(lexer, log)?;
                match end {
                    Some((close, Token::ClosingParenthesis)) => {
                        Expr::new(range + close, ExprNode::Invocation(name, vec, map))
                    }
                    other => {
                        let range = other.map(|(range, _)| range);
                        return Err(Error::UnclosedBracketUntil(open, range));
                    }
                }
            }
            // Identifier
            other => return Ok((Expr::new(range, ExprNode::Identifier(name)).into(), other)),
        },
        Some((range, Token::Parameter(name))) => Expr::new(range, ExprNode::Parameter(name)),
        Some((range, Token::Number(value))) => Expr::new(range, ExprNode::Number(value)),
        Some((range, Token::String(string))) => Expr::new(range, ExprNode::String(string)),
        // 前置 `-` （負号）
        Some((op, Token::Minus)) => {
            let mut ret = parse_print(lexer, log)?;
            ret.0 = match ret.0 {
                Some(expr) => Expr::new(op + &expr.range, ExprNode::Minus(expr.into())).into(),
                None => return Err(Error::EmptyOperandUnary(op)),
            };
            return Ok(ret);
        }
        // 前置 `/` （逆数）
        Some((op, Token::Slash)) => {
            let mut ret = parse_print(lexer, log)?;
            ret.0 = match ret.0 {
                Some(expr) => Expr::new(op + &expr.range, ExprNode::Reciprocal(expr.into())).into(),
                None => return Err(Error::EmptyOperandUnary(op)),
            };
            return Ok(ret);
        }
        // 前置 `!` （否定）
        Some((op, Token::Exclamation)) => {
            let mut ret = parse_print(lexer, log)?;
            ret.0 = match ret.0 {
                Some(expr) => Expr::new(op + &expr.range, ExprNode::Not(expr.into())).into(),
                None => return Err(Error::EmptyOperandUnary(op)),
            };
            return Ok(ret);
        }
        // 丸括弧でくくられた部分
        Some((open, Token::OpeningParenthesis)) => match parse_expression(lexer, log)? {
            (expr, Some((close, Token::ClosingParenthesis))) => match expr {
                Some(expr) => Expr::new(open + close, ExprNode::Group(expr.into())),
                None => return Err(Error::EmptyParentheses(open, close)),
            },
            (_, other) => {
                let range = other.map(|(range, _)| range);
                return Err(Error::UnclosedBracketUntil(open, range));
            }
        },
        Some((open, Token::OpeningBracket)) => {
            // 角括弧でくくられた部分（ Score ）
            todo!();
        }
        other => return Ok((None, other)),
    };
    Ok((expression.into(), lexer.next(log)?))
}

/// 後置 `?` （出力）
///
/// 前置演算子 `-` `/` `!` より優先順位は高い
fn parse_print(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<Parsed<Option<Expr>>, Error> {
    let mut ret = parse_factor(lexer, log)?;
    if let (Some(mut expr), mut end) = ret {
        while let Some((op, Token::Question)) = end {
            expr = Expr::new(&expr.range + op, ExprNode::Print(expr.into()));
            end = lexer.next(log)?;
        }
        ret = (Some(expr), end);
    }
    Ok(ret)
}

macro_rules! def_binary_operator {
    ($prev:ident => $next:ident: $($from:path => $to:expr),* $(,)?) => {
        pub fn $next(
            lexer: &mut lexer::Lexer<impl BufRead>,
            log: &mut Vec<String>,
        ) -> Result<Parsed<Option<Expr>>, Error> {
            let mut ret = $prev(lexer, log)?;
            if let (Some(mut expr), mut end) = ret {
                loop {
                    match end {
                        $(Some((op, $from)) => match $prev(lexer, log)? {
                            (Some(right), token) => {
                                expr = Expr::new(
                                    &expr.range + &right.range,
                                    $to(expr.into(), right.into()),
                                );
                                end = token;
                            }
                            (None, _) => return Err(Error::EmptyOperandRight(op)),
                        })*
                        _ => break ret = (Some(expr), end),
                    }
                }
            };
            Ok(ret)
        }
    };
}

def_binary_operator! {
    parse_factor => parse_operator1:
        Token::DoubleLess => ExprNode::LeftShift,
        Token::DoubleGreater => ExprNode::RightShift,
}
def_binary_operator! {
    parse_operator1 => parse_operator2:
        Token::Circumflex => ExprNode::Pow,
}
def_binary_operator! {
    parse_operator2 => parse_operator3:
        Token::Asterisk => ExprNode::Mul,
        Token::Slash => ExprNode::Div,
        Token::Percent => ExprNode::Rem,
}
def_binary_operator! {
    parse_operator3 => parse_operator4:
        Token::Plus => ExprNode::Add,
        Token::Minus => ExprNode::Sub,
}
def_binary_operator! {
    parse_operator4 => parse_operator5:
        Token::Less => ExprNode::Less,
        Token::Greater => ExprNode::Greater,
}
def_binary_operator! {
    parse_operator5 => parse_operator6:
        Token::DoubleEqual => ExprNode::Equal,
        Token::ExclamationEqual => ExprNode::NotEqual,
}
def_binary_operator! {
    parse_operator6 => parse_expression:
        Token::DoubleAmpersand => ExprNode::And,
        Token::DoubleBar => ExprNode::Or,
}

fn parse_invocation_arguments(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<Parsed<(Vec<Expr>, HashMap<String, Expr>)>, Error> {
    let mut vec = Vec::new();
    let mut map = HashMap::new();
    loop {
        let (item, end) = parse_expression(lexer, log)?;
        match end {
            Some((comma, Token::Comma)) => vec.push(item.ok_or(Error::EmptyArgument(comma))?),
            Some((equal, Token::Equal)) => match item {
                Some(Expr {
                    node: ExprNode::Identifier(name),
                    ..
                }) => {
                    let (item, end) = parse_expression(lexer, log)?;
                    map.insert(name, item.ok_or(Error::EmptyNamedArgument(equal))?);
                    if !matches!(end, Some((_, Token::Comma))) {
                        return Ok(((vec, map), end));
                    }
                }
                other => {
                    let range = other.map(|Expr { range, .. }| range);
                    return Err(Error::ArgumentNameNotIdentifier(range, equal));
                }
            },
            _ => {
                if let Some(item) = item {
                    vec.push(item);
                }
                return Ok(((vec, map), end));
            }
        }
    }
}

/*

fn parse_list1(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> ResultAndNext<Vec<Expr>> {
    let mut vec = Vec::new();
    loop {
        let (item, end) = parse_expression(lexer, log)?;
        vec.push(item);
        if !matches!(end, Some((_, Token::Comma))) {
            return Ok((vec, end));
        }
    }
}
fn parse_list(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> ResultAndNext<Vec<Vec<Expr>>> {
    let mut vec = Vec::new();
    loop {
        let (item, end) = parse_list1(lexer, log)?;
        vec.push(item);
        if !matches!(end, Some((_, Token::Semicolon))) {
            return Ok((vec, end));
        }
    }
}
*/

pub fn parse_statement_or_token(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<Result<Stmt, Option<(pos::Range, Token)>>, Error> {
    todo!();
}

/*
pub fn parse_statement_or_token(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<Result<Stmt, Option<(pos::Range, Token)>>, Error> {
    match parse_expression(lexer, log)? {
        (expr, Some((_, Token::Semicolon))) => Ok(Ok(Stmt::Expr(expr))),
        (lhs, Some((_, Token::Equal))) => {
            let name = match lhs.try_into_identifier() {
                Ok(name) => name,
                Err(lhs) => todo!(),
            };
            match parse_expression(lexer, log)? {
                (expr, Some((_, Token::Semicolon))) => Ok(Ok(Statement::Substitution(name, expr))),
                _ => todo!(),
            }
        }
        (Expr(None), Some((_, Token::OpeningBrace))) => {
            let mut vec = Vec::new();
            loop {
                match parse_statement_or_token(lexer, log)? {
                    Ok(stmt) => vec.push(stmt),
                    Err(Some((_, Token::ClosingBrace))) => break Ok(Ok(Statement::Block(vec))),
                    Err(other) => {
                        todo!();
                    }
                }
            }
        }
        (Expr(None), Some((_, Token::KeywordBreak))) => Ok(Ok(Statement::Break)),
        (Expr(None), Some((_, Token::KeywordContinue))) => Ok(Ok(Statement::Continue)),
        (Expr(None), other) => Ok(Err(other)),
        (Expr(Some((range, _))), _) => Err(Error::NoSemicolonAtEndOfStatement(range)),
    }
}

/// ファイル終端に達したら None
pub fn parse_statement(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<Option<Stmt>, Error> {
    todo!();
}
*/
