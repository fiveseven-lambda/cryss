//! トークンを抽象構文木に変換する．

use crate::{error, lexer, pos, syntax, token};

/// パースした式と，その直後のトークン
type Result<T> = std::result::Result<(T, Option<(pos::Range, token::Token)>), error::Error>;

fn parse_factor(
    lexer: &mut lexer::Lexer<impl std::io::BufRead>,
    log: &mut Vec<String>,
) -> Result<syntax::Expression> {
    let (range, node) = match lexer.next(log)? {
        Some((range, token::Token::Identifier(name))) => (range, syntax::Node::Identifier(name)),
        Some((range, token::Token::Parameter(name))) => (range, syntax::Node::Parameter(name)),
        Some((range, token::Token::Number(value))) => (range, syntax::Node::Number(value)),
        Some((range, token::Token::String(string))) => (range, syntax::Node::String(string)),
        Some((range, token::Token::Minus)) => {
            return parse_factor(lexer, log).map(|(expr, end)| {
                (
                    syntax::Expression::some(
                        range + expr.range(),
                        syntax::Node::Minus(expr.into()),
                    ),
                    end,
                )
            })
        }
        other => return Ok((syntax::Expression::empty(), other)),
    };
    todo!();
}
