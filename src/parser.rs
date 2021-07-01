//! トークンを抽象構文木に変換する．

use crate::{error, lexer, pos, syntax, token};
use syntax::{Expression, ExpressionNode, Statement};
use token::Token;

use std::collections::HashMap;
use std::io::BufRead;

// 関数呼び出し構文において
// `fnc()`: 引数 0 個，カンマ 0 個
// `fnc(1)`: 引数 1 個，カンマ 0 個
// `fnc(1,)`: 引数 1 個，カンマ 1 個
// `fnc(1, 2)`: 引数 2 個，カンマ 1 個
// `fnc(1, 2,)`: 引数 3 個，カンマ 2 個

/// パースしたものと，その直後のトークン
type Parsed<T> = (T, Option<(pos::Range, Token)>);

fn parse_factor(lexer: &mut lexer::Lexer<impl BufRead>, log: &mut Vec<String>) -> Result<Parsed<Option<Expression>>, error::Error> {
    let expression = match lexer.next(log)? {
        Some((range, Token::Identifier(name))) => match lexer.next(log)? {
            Some((range, Token::OpeningParenthesis)) => {
                // 関数呼び出し
                todo!();
            }
            other => return Ok((Expression::new(range, ExpressionNode::Identifier(name)).into(), other)),
        },
        Some((range, Token::Parameter(name))) => Expression::new(range, ExpressionNode::Parameter(name)),
        Some((range, Token::Number(value))) => Expression::new(range, ExpressionNode::Number(value)),
        Some((range, Token::String(string))) => Expression::new(range, ExpressionNode::String(string)),
        Some((range, Token::Minus)) => match parse_print(lexer, log)? {
            (Some(expr), end) => return Ok((Expression::new(range, ExpressionNode::Minus(expr.into())).into(), end)),
            (None, _) => return Err(error::Error::EmptyOperandUnary(range)),
        },
        Some((range, Token::Slash)) => match parse_print(lexer, log)? {
            (Some(expr), end) => return Ok((Expression::new(range, ExpressionNode::Reciprocal(expr.into())).into(), end)),
            (None, _) => return Err(error::Error::EmptyOperandUnary(range)),
        },
        Some((range, Token::Exclamation)) => match parse_print(lexer, log)? {
            (Some(expr), end) => return Ok((Expression::new(range, ExpressionNode::Not(expr.into())).into(), end)),
            (None, _) => return Err(error::Error::EmptyOperandUnary(range)),
        },
        Some((range, Token::OpeningParenthesis)) => {
            // 丸括弧でくくられた部分（ Group ）
            todo!();
        }
        Some((range, Token::OpeningBracket)) => {
            // 角括弧でくくられた部分（ Score ）
            todo!();
        }
        other => return Ok((None, other)),
    };
    Ok((expression.into(), lexer.next(log)?))
}

pub fn parse_print(lexer: &mut lexer::Lexer<impl BufRead>, log: &mut Vec<String>) -> Result<Parsed<Option<Expression>>, error::Error> {
    let mut ret = parse_factor(lexer, log)?;
    if let (Some(mut expr), mut end) = ret {
        while let Some((range, Token::Question)) = end {
            expr = Expression::new(expr.range.clone() + range, ExpressionNode::Print(expr.into()));
            end = lexer.next(log)?;
        }
        ret = (Some(expr), end);
    }
    Ok(ret)
}

/*

macro_rules! def_binary_operator {
    ($prev:ident => $next:ident: $($from:path => $to:expr),* $(,)?) => {
        fn $next(
            lexer: &mut lexer::Lexer<impl BufRead>,
            log: &mut Vec<String>,
        ) -> ResultAndNext<Expression> {
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
    parse_operator6 => parse_expression:
        Token::DoubleAmpersand => Node::And,
        Token::DoubleBar => Node::Or,
}

fn parse_invocation_arguments(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> ResultAndNext<(Vec<Expression>, HashMap<String, Expression>)> {
    let mut vec = Vec::new();
    let mut map = HashMap::new();
    loop {
        let (item, end) = parse_expression(lexer, log)?;
        match end {
            Some((_, Token::Comma)) => vec.push(item),
            Some((equal, Token::Equal)) => {
                let name = match item.try_into_identifier() {
                    Ok(name) => name,
                    Err(item) => {
                        return Err(error::Error::ArgumentNameNotIdentifier(item.range(), equal))
                    }
                };
                let (item, end) = parse_expression(lexer, log)?;
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

fn parse_list1(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> ResultAndNext<Vec<Expression>> {
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
) -> ResultAndNext<Vec<Vec<Expression>>> {
    let mut vec = Vec::new();
    loop {
        let (item, end) = parse_list1(lexer, log)?;
        vec.push(item);
        if !matches!(end, Some((_, Token::Semicolon))) {
            return Ok((vec, end));
        }
    }
}

pub fn parse_statement_or_token(
    lexer: &mut lexer::Lexer<impl BufRead>,
    log: &mut Vec<String>,
) -> Result<Result<Statement, Option<(pos::Range, Token)>>, error::Error> {
    match parse_expression(lexer, log)? {
        (expr, Some((_, Token::Semicolon))) => Ok(Ok(Statement::Expression(expr))),
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
        (Expression(None), Some((_, Token::OpeningBrace))) => {
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
        (Expression(None), Some((_, Token::KeywordBreak))) => Ok(Ok(Statement::Break)),
        (Expression(None), Some((_, Token::KeywordContinue))) => Ok(Ok(Statement::Continue)),
        (Expression(None), other) => Ok(Err(other)),
        (Expression(Some((range, _))), _) => Err(error::Error::NoSemicolonAtEndOfStatement(range)),
    }
}
*/

/// ファイル終端に達したら None
pub fn parse_statement(lexer: &mut lexer::Lexer<impl BufRead>, log: &mut Vec<String>) -> Result<Option<Statement>, error::Error> {
    todo!();
}
