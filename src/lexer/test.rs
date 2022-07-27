#![cfg(test)]

use super::Lexer;
use crate::token::Token;

#[test]
fn identifier() {
    let input: &[_] = b"abc a00 _0123456789$";
    let mut lexer = Lexer::new(Box::new(input), false);
    for &ans in &["abc", "a00", "_0123456789$"] {
        let next = match lexer.next() {
            Ok(item) => item,
            Err(err) => {
                err.eprint(&lexer.log);
                panic!();
            }
        };
        let (_, token) = next.unwrap();
        match token {
            Token::Identifier(s) => assert_eq!(s, ans),
            _ => panic!("not an identifier"),
        }
    }
}
