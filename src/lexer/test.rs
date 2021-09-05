#![cfg(test)]

use crate::lexer::Lexer;
use crate::token;

#[test]
fn identifier() {
    let input: &[u8] = b"foo bar
    _0123456789$";
    let mut log = Vec::new();
    let mut lexer = Lexer::new(Box::new(input), false);
    let mut iter =
        std::iter::from_fn(|| lexer.next(&mut log).ok().unwrap()).map(|(_, token)| token);
    for &ans in &["foo", "bar", "_0123456789$"] {
        match iter.next().unwrap() {
            token::Token::Identifier(s) => assert_eq!(s, ans),
            _ => panic!(),
        }
    }
    assert!(iter.next().is_none());
}

#[test]
fn literals() {
    let input: &[u8] = br#"
    10 0b10 0o76543210 0xfFeEdDcCbBaA9876543210
    123. .123
    123e10 123e+10 123e-10
    123.e10 .123e10 123.456e10
    "foo
bar"
    "\n\r\t\0\"\a""#;
    let mut log = Vec::new();
    let mut lexer = Lexer::new(Box::new(input), false);
    let mut iter =
        std::iter::from_fn(|| lexer.next(&mut log).ok().unwrap()).map(|(_, token)| token);
    for &ans in &["10", "0b10", "0o76543210", "0xfFeEdDcCbBaA9876543210"] {
        match iter.next().unwrap() {
            token::Token::Integer(s) => assert_eq!(s, ans),
            _ => panic!(),
        }
    }
    for &ans in &[
        "123.",
        ".123",
        "123e10",
        "123e+10",
        "123e-10",
        "123.e10",
        ".123e10",
        "123.456e10",
    ] {
        match iter.next().unwrap() {
            token::Token::Real(s) => assert_eq!(s, ans),
            _ => panic!(),
        }
    }
    for &ans in &["foo\nbar", "\n\r\t\0\"a"] {
        match iter.next().unwrap() {
            token::Token::String(s) => assert_eq!(s, ans),
            _ => panic!(),
        }
    }
    assert!(iter.next().is_none());
}

#[test]
fn operators() {
    let input: &[u8] = b"+-***/%===!!=<=<<<<<>=>>>>><>&&&|||^.:;,?()[]{}";
    let mut log = Vec::new();
    let mut lexer = Lexer::new(Box::new(input), false);
    let mut iter =
        std::iter::from_fn(|| lexer.next(&mut log).ok().unwrap()).map(|(_, token)| token);
    assert!(matches!(iter.next(), Some(token::Token::Plus)));
    assert!(matches!(iter.next(), Some(token::Token::Hyphen)));
    assert!(matches!(iter.next(), Some(token::Token::DoubleAsterisk)));
    assert!(matches!(iter.next(), Some(token::Token::Asterisk)));
    assert!(matches!(iter.next(), Some(token::Token::Slash)));
    assert!(matches!(iter.next(), Some(token::Token::Percent)));
    assert!(matches!(iter.next(), Some(token::Token::DoubleEqual)));
    assert!(matches!(iter.next(), Some(token::Token::Equal)));
    assert!(matches!(iter.next(), Some(token::Token::Exclamation)));
    assert!(matches!(iter.next(), Some(token::Token::ExclamationEqual)));
    assert!(matches!(iter.next(), Some(token::Token::LessEqual)));
    assert!(matches!(iter.next(), Some(token::Token::TripleLess)));
    assert!(matches!(iter.next(), Some(token::Token::DoubleLess)));
    assert!(matches!(iter.next(), Some(token::Token::GreaterEqual)));
    assert!(matches!(iter.next(), Some(token::Token::TripleGreater)));
    assert!(matches!(iter.next(), Some(token::Token::DoubleGreater)));
    assert!(matches!(iter.next(), Some(token::Token::Less)));
    assert!(matches!(iter.next(), Some(token::Token::Greater)));
    assert!(matches!(iter.next(), Some(token::Token::DoubleAmpersand)));
    assert!(matches!(iter.next(), Some(token::Token::Ampersand)));
    assert!(matches!(iter.next(), Some(token::Token::DoubleBar)));
    assert!(matches!(iter.next(), Some(token::Token::Bar)));
    assert!(matches!(iter.next(), Some(token::Token::Circumflex)));
    assert!(matches!(iter.next(), Some(token::Token::Dot)));
    assert!(matches!(iter.next(), Some(token::Token::Colon)));
    assert!(matches!(iter.next(), Some(token::Token::Semicolon)));
    assert!(matches!(iter.next(), Some(token::Token::Comma)));
    assert!(matches!(iter.next(), Some(token::Token::Question)));
    assert!(matches!(
        iter.next(),
        Some(token::Token::OpeningParenthesis)
    ));
    assert!(matches!(
        iter.next(),
        Some(token::Token::ClosingParenthesis)
    ));
    assert!(matches!(iter.next(), Some(token::Token::OpeningBracket)));
    assert!(matches!(iter.next(), Some(token::Token::ClosingBracket)));
    assert!(matches!(iter.next(), Some(token::Token::OpeningBrace)));
    assert!(matches!(iter.next(), Some(token::Token::ClosingBrace)));
    assert!(iter.next().is_none());
}

#[test]
fn comments() {
    let input: &[u8] = b"hello//
how/* comment */are/* comment **/you
/*/ nested /* comments **//* are *// available */
    /* // */ line comment can also be nested
    in block comment */";
    let mut log = Vec::new();
    let mut lexer = Lexer::new(Box::new(input), false);
    let mut iter =
        std::iter::from_fn(|| lexer.next(&mut log).ok().unwrap()).map(|(_, token)| token);
    for &ans in &["hello", "how", "are", "you"] {
        match iter.next().unwrap() {
            token::Token::Identifier(s) => assert_eq!(s, ans),
            _ => panic!(),
        }
    }
    assert!(iter.next().is_none());
}
