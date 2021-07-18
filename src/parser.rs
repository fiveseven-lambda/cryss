//! トークン（ `mod token` ）を抽象構文木（ `mod syntax` ）に変換する．

use crate::{error, lexer, pos, syntax, token};
use error::Error;
use syntax::{Expression, Node, Statement};
use token::Token;

use std::collections::HashMap;

/// パースしたものと，その直後のトークン
type Parsed<T> = (T, Option<(pos::Range, Token)>);

/// 識別子，リテラル，関数呼び出し，括弧など，
/// あと前置演算子 `-` `/` `!`
fn parse_factor(
    lexer: &mut lexer::Lexer,
    log: &mut Vec<String>,
) -> Result<Parsed<Option<Expression>>, Error> {
    let expression = match lexer.next(log)? {
        Some((range, Token::Identifier(name))) => match lexer.next(log)? {
            // 関数呼び出し
            Some((open, Token::OpeningParenthesis)) => {
                let ((vec, map), end) = parse_invocation_arguments(lexer, log)?;
                match end {
                    Some((close, Token::ClosingParenthesis)) => {
                        Expression::new(range + close, Node::Invocation(name, vec, map))
                    }
                    Some((range, _)) => return Err(Error::UnclosedBracketUntil(open, range)),
                    None => return Err(Error::UnclosedBracketUntilEOF(open)),
                }
            }
            // Identifier
            other => return Ok((Expression::new(range, Node::Identifier(name)).into(), other)),
        },
        Some((range, Token::Parameter(name))) => Expression::new(range, Node::Parameter(name)),
        Some((range, Token::Number(value))) => Expression::new(range, Node::Number(value)),
        Some((range, Token::String(string))) => Expression::new(range, Node::String(string)),
        // 前置 `-` （負号）
        Some((op, Token::Hyphen)) => {
            let mut ret = parse_print(lexer, log)?;
            ret.0 = match ret.0 {
                Some(expr) => Expression::new(op + &expr.range, Node::Minus(expr.into())).into(),
                None => return Err(Error::EmptyOperandUnary(op)),
            };
            return Ok(ret);
        }
        // 前置 `/` （逆数）
        Some((op, Token::Slash)) => {
            let mut ret = parse_print(lexer, log)?;
            ret.0 = match ret.0 {
                Some(expr) => {
                    Expression::new(op + &expr.range, Node::Reciprocal(expr.into())).into()
                }
                None => return Err(Error::EmptyOperandUnary(op)),
            };
            return Ok(ret);
        }
        // 前置 `!` （否定）
        Some((op, Token::Exclamation)) => {
            let mut ret = parse_print(lexer, log)?;
            ret.0 = match ret.0 {
                Some(expr) => Expression::new(op + &expr.range, Node::Not(expr.into())).into(),
                None => return Err(Error::EmptyOperandUnary(op)),
            };
            return Ok(ret);
        }
        // 丸括弧でくくられた部分
        Some((open, Token::OpeningParenthesis)) => match parse_expression(lexer, log)? {
            (expr, Some((close, Token::ClosingParenthesis))) => match expr {
                Some(expr) => Expression::new(open + close, Node::Group(expr.into())),
                None => return Err(Error::EmptyParentheses(open, close)),
            },
            (_, Some((range, _))) => return Err(Error::UnclosedBracketUntil(open, range)),
            (_, None) => return Err(Error::UnclosedBracketUntilEOF(open)),
        },
        Some((open, Token::OpeningBracket)) => match parse_list2(lexer, log)? {
            (list, Some((close, Token::ClosingBracket))) => {
                Expression::new(open + close, Node::Score(list))
            }
            (_, Some((range, _))) => return Err(Error::UnclosedBracketUntil(open, range)),
            (_, None) => return Err(Error::UnclosedBracketUntilEOF(open)),
        },
        other => return Ok((None, other)),
    };
    Ok((expression.into(), lexer.next(log)?))
}

/// 後置 `?` （出力）
///
/// 前置演算子 `-` `/` `!` より優先順位は高い
fn parse_print(
    lexer: &mut lexer::Lexer,
    log: &mut Vec<String>,
) -> Result<Parsed<Option<Expression>>, Error> {
    let mut ret = parse_factor(lexer, log)?;
    if let (Some(mut expr), mut end) = ret {
        while let Some((op, Token::Question)) = end {
            expr = Expression::new(&expr.range + op, Node::Print(expr.into()));
            end = lexer.next(log)?;
        }
        ret = (Some(expr), end);
    }
    Ok(ret)
}

macro_rules! def_binary_operator {
    ($(/ $doc:tt)* $prev:ident => $next:ident: $($from:path => $to:expr),* $(,)?) => {
        $(#[doc = $doc])*
        fn $next(
            lexer: &mut lexer::Lexer,
            log: &mut Vec<String>,
        ) -> Result<Parsed<Option<Expression>>, Error> {
            let mut ret = $prev(lexer, log)?;
            if let (Some(mut expr), mut end) = ret {
                loop {
                    match end {
                        $(Some((op, $from)) => match $prev(lexer, log)? {
                            (Some(right), token) => {
                                expr = Expression::new(
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
    / "左シフト `<<`，"
    / "右シフト `>>`"
    / ""
    / "Sound の時間をズラす"
    parse_print => parse_operator1:
        Token::DoubleLess => Node::LeftShift,
        Token::DoubleGreater => Node::RightShift,
}
def_binary_operator! {
    / "累乗 `^`"
    parse_operator1 => parse_operator2:
        Token::Circumflex => Node::Pow,
}
def_binary_operator! {
    / "掛け算 `*`，"
    / "割り算 `/`，"
    / "余り `%`"
    parse_operator2 => parse_operator3:
        Token::Asterisk => Node::Mul,
        Token::Slash => Node::Div,
        Token::Percent => Node::Rem,
}
def_binary_operator! {
    / "足し算 `+`，"
    / "引き算 `-`"
    parse_operator3 => parse_operator4:
        Token::Plus => Node::Add,
        Token::Hyphen => Node::Sub,
}
def_binary_operator! {
    / "比較演算子 `<`, `>`"
    parse_operator4 => parse_operator5:
        Token::Less => Node::Less,
        Token::Greater => Node::Greater,
}
def_binary_operator! {
    / "比較演算子 `==`, `!=`"
    parse_operator5 => parse_operator6:
        Token::DoubleEqual => Node::Equal,
        Token::ExclamationEqual => Node::NotEqual,
}
def_binary_operator! {
    / "かつ `&&`，"
    / "または `||`"
    parse_operator6 => parse_expression:
        Token::DoubleAmpersand => Node::And,
        Token::DoubleBar => Node::Or,
}

/// 関数呼び出しにおける引数
///
/// 名前なし引数（ expr 形式）と
/// 名前つき引数（ identifier `=` expr の形式）
///
/// 最後のカンマはあってもなくてもいい
#[allow(clippy::type_complexity)]
fn parse_invocation_arguments(
    lexer: &mut lexer::Lexer,
    log: &mut Vec<String>,
) -> Result<Parsed<(Vec<Expression>, HashMap<String, Expression>)>, Error> {
    let mut vec = Vec::new();
    let mut map = HashMap::new();
    loop {
        let (item, end) = parse_expression(lexer, log)?;
        match end {
            Some((comma, Token::Comma)) => vec.push(item.ok_or(Error::EmptyArgument(comma))?),
            Some((equal, Token::Equal)) => {
                match item {
                    Some(expr) => match expr.node {
                        Node::Identifier(name) => {
                            let (item, end) = parse_expression(lexer, log)?;
                            map.insert(name, item.ok_or(Error::EmptyNamedArgument(equal))?);
                            if !matches!(end, Some((_, Token::Comma))) {
                                return Ok(((vec, map), end));
                            }
                        }
                        _ => return Err(Error::InvalidArgumentName(expr.range, equal)),
                    },
                    None => return Err(Error::EmptyArgumentName(equal.clone())),
                };
            }
            _ => {
                vec.extend(item);
                return Ok(((vec, map), end));
            }
        }
    }
}

/// カンマ区切り（空の要素は無視）
fn parse_list1(
    lexer: &mut lexer::Lexer,
    log: &mut Vec<String>,
) -> Result<Parsed<Vec<Expression>>, Error> {
    let mut vec = Vec::new();
    loop {
        let (item, end) = parse_expression(lexer, log)?;
        vec.extend(item);
        if !matches!(end, Some((_, Token::Comma))) {
            return Ok((vec, end));
        }
    }
}

/// セミコロン区切り（空のものは空の Vec として追加）
fn parse_list2(
    lexer: &mut lexer::Lexer,
    log: &mut Vec<String>,
) -> Result<Parsed<Vec<Vec<Expression>>>, Error> {
    let mut vec = Vec::new();
    loop {
        let (item, end) = parse_list1(lexer, log)?;
        vec.push(item);
        if !matches!(end, Some((_, Token::Semicolon))) {
            return Ok((vec, end));
        }
    }
}

pub fn parse_statement(
    lexer: &mut lexer::Lexer,
    log: &mut Vec<String>,
) -> Result<Option<Statement>, Error> {
    let ret = match parse_expression(lexer, log)? {
        (Some(expr), Some((_, Token::Semicolon))) => Statement::Expression(expr.into()),
        (None, Some((_, Token::Semicolon))) => Statement::Expression(None),
        (Some(lhs), Some((equal, Token::Equal))) => {
            let name = match lhs.node {
                Node::Identifier(name) => name,
                _ => return Err(Error::LHSNotIdentifier(lhs.range, equal)),
            };
            match parse_expression(lexer, log)? {
                (Some(expr), Some((_, Token::Semicolon))) => {
                    Statement::Substitution(lhs.range, name, expr)
                }
                (None, _) => return Err(Error::EmptyRHS(equal)),
                (Some(expr), _) => return Err(Error::NoSemicolonAtEndOfStatement(expr.range)),
            }
        }
        (None, Some((r#let, Token::KeywordLet))) => {
            let (range, name) = match lexer.next(log)? {
                Some((range, Token::Identifier(name))) => (range, name),
                Some((range, _)) => return Err(Error::UnexpectedTokenAfterKeyword(r#let, range)),
                None => return Err(Error::UnexpectedEOFAfterKeyword(r#let)),
            };
            let equal = match lexer.next(log)? {
                Some((equal, Token::Equal)) => equal,
                _ => return Err(Error::NoSubstitutionAfterLet(r#let)),
            };
            match parse_expression(lexer, log)? {
                (Some(expr), Some((_, Token::Semicolon))) => {
                    Statement::Declaration(range, name, expr)
                }
                (None, _) => return Err(Error::EmptyRHS(equal)),
                (Some(expr), _) => return Err(Error::NoSemicolonAtEndOfStatement(expr.range)),
            }
        }
        (None, Some((def, Token::KeywordDef))) => {
            let (range, name) = match lexer.next(log)? {
                Some((range, Token::Identifier(name))) => (range, name),
                Some((other, _)) => return Err(Error::UnexpectedTokenAfterKeyword(def, other)),
                None => return Err(Error::UnexpectedEOFAfterKeyword(def)),
            };
            todo!();
        }
        (None, Some((r#if, Token::KeywordIf))) => {
            let open = match lexer.next(log)? {
                Some((open, Token::OpeningParenthesis)) => open,
                Some((other, _)) => return Err(Error::UnexpectedTokenAfterKeyword(r#if, other)),
                None => return Err(Error::UnexpectedEOFAfterKeyword(r#if)),
            };
            let (condition, close) = match parse_expression(lexer, log)? {
                (Some(expr), Some((close, Token::ClosingParenthesis))) => (expr, close),
                (_, Some((range, _))) => return Err(Error::UnclosedBracketUntil(open, range)),
                (_, None) => return Err(Error::UnclosedBracketUntilEOF(open)),
            };
            let body_if = parse_statement(lexer, log)?
                .ok_or(Error::UnexpectedEOFAfterCondition(r#if, open + close))?;
            let body_else = lexer
                .ask(|token| matches!(token, Token::KeywordElse), log)?
                .then(|| {
                    let (r#else, _) = lexer.next(log)?.unwrap();
                    match parse_statement(lexer, log)? {
                        Some(statement) => Ok(statement.into()),
                        None => Err(Error::UnexpectedEOFAfterKeyword(r#else)),
                    }
                })
                .transpose()?;
            Statement::If(condition, body_if.into(), body_else)
        }
        (None, Some((r#while, Token::KeywordWhile))) => {
            let open = match lexer.next(log)? {
                Some((open, Token::OpeningParenthesis)) => open,
                Some((other, _)) => return Err(Error::UnexpectedTokenAfterKeyword(r#while, other)),
                None => return Err(Error::UnexpectedEOFAfterKeyword(r#while)),
            };
            let (condition, close) = match parse_expression(lexer, log)? {
                (Some(expr), Some((close, Token::ClosingParenthesis))) => (expr, close),
                (_, Some((range, _))) => return Err(Error::UnclosedBracketUntil(open, range)),
                (_, None) => return Err(Error::UnclosedBracketUntilEOF(open)),
            };
            let body = parse_statement(lexer, log)?
                .ok_or(Error::UnexpectedEOFAfterCondition(r#while, open + close))?;
            Statement::While(condition, body.into())
        }
        (None, Some((open, Token::OpeningBrace))) => {
            let mut vec = Vec::new();
            while !lexer.ask(|token| matches!(token, Token::ClosingBrace), log)? {
                match parse_statement(lexer, log)? {
                    Some(statement) => vec.push(statement),
                    None => return Err(Error::UnclosedBracketUntilEOF(open)),
                }
            }
            lexer.next(log)?;
            Statement::Block(vec)
        }
        (None, Some((r#break, Token::KeywordBreak))) => match lexer.next(log)? {
            Some((_, Token::Semicolon)) => Statement::Break(r#break),
            Some((other, _)) => return Err(Error::UnexpectedTokenAfterKeyword(r#break, other)),
            None => return Err(Error::UnexpectedEOFAfterKeyword(r#break)),
        },
        (None, Some((r#continue, Token::KeywordContinue))) => match lexer.next(log)? {
            Some((_, Token::Semicolon)) => Statement::Continue(r#continue),
            Some((other, _)) => return Err(Error::UnexpectedTokenAfterKeyword(r#continue, other)),
            None => return Err(Error::UnexpectedEOFAfterKeyword(r#continue)),
        },
        (None, Some((r#return, Token::KeywordReturn))) => match parse_expression(lexer, log)? {
            (expr, Some((_, Token::Semicolon))) => Statement::Return(r#return, expr),
            (Some(other), _) => return Err(Error::NoSemicolonAtEndOfStatement(other.range)),
            (None, _) => return Err(Error::UnexpectedEOFAfterKeyword(r#return)),
        },
        (Some(expr), _) => return Err(Error::NoSemicolonAtEndOfStatement(expr.range)),
        (None, None) => return Ok(None),
        (None, Some((range, _))) => return Err(Error::UnexpectedToken(range)),
    };
    Ok(Some(ret))
}
