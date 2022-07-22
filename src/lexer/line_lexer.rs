use crate::error::Error;
use crate::pos;
use crate::token;
use std::collections::VecDeque;
pub struct LineLexer {}

impl LineLexer {
    pub fn new() -> LineLexer {
        LineLexer {}
    }

    pub fn run(
        &mut self,
        line_num: usize,
        line: &str,
        tokens: &mut VecDeque<token::RToken>,
    ) -> Result<(), Error> {
        let mut iter = line.char_indices().peekable();

        loop {
            let (start, first) = loop {
                match iter.next() {
                    Some((_, c)) if c.is_ascii_whitespace() => {}
                    Some(item) => break item,
                    None => return Ok(()),
                }
            };
            let range_gen = |end: Option<&(usize, char)>| {
                pos::Range::from_line_byte(line_num, start, end.map(|&(i, _)| i))
            };
            let token = match first {
                'a'..='z' | 'A'..='Z' | '_' | '$' => {
                    let mut s = first.to_string();
                    while let Some(&(_, c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$'))) =
                        iter.peek()
                    {
                        s.push(c);
                        iter.next();
                    }
                    token::Token::Identifier(s)
                }
                '0'..='9' | '.' => {
                    enum State {
                        Zero,
                        BinInt(String),
                        OctInt(String),
                        DecInt(String),
                        HexInt(String),
                        Dot,
                        Decimal(String),
                        SciE(String),
                        SciSign(String),
                        Sci(String),
                    }
                    let mut state = match first {
                        '.' => State::Dot,
                        '0' => State::Zero,
                        _ => State::DecInt(first.into()),
                    };
                    loop {
                        fn append_char(mut s: String, c: char) -> String {
                            s.push(c);
                            s
                        }
                        let c = match iter.peek() {
                            Some(&(_, c)) => c,
                            None => break,
                        };
                        state = match (state, c) {
                            (State::Dot, '0'..='9') => State::Decimal(format!(".{c}")),
                            (State::Zero, '_') => State::DecInt("0".to_string()),
                            (State::Zero, 'b') => State::BinInt(String::new()),
                            (State::Zero, 'o') => State::OctInt(String::new()),
                            (State::Zero, 'x') => State::HexInt(String::new()),
                            (State::Zero, '.') => State::Decimal("0.".to_string()),
                            (State::Zero, '0'..='9') => State::Decimal(c.to_string()),
                            (State::DecInt(s), '.') => State::Decimal(append_char(s, '.')),
                            (State::Decimal(s), '0'..='9') => State::Decimal(append_char(s, c)),
                            (State::BinInt(s), '0'..='9') => State::BinInt(append_char(s, c)),
                            (State::OctInt(s), '0'..='9') => State::OctInt(append_char(s, c)),
                            (State::DecInt(s), '0'..='9') => State::DecInt(append_char(s, c)),
                            (State::HexInt(s), '0'..='9' | 'a'..='f' | 'A'..='F') => {
                                State::HexInt(append_char(s, c))
                            }
                            (State::Zero, 'e' | 'E') => State::SciE(format!("0{c}")),
                            (State::DecInt(s) | State::Decimal(s), 'e' | 'E') => {
                                State::SciE(append_char(s, c))
                            }
                            (State::SciE(s), '+' | '-') => State::SciSign(append_char(s, c)),
                            (State::SciE(s) | State::SciSign(s) | State::Sci(s), '0'..='9') => {
                                State::Sci(append_char(s, c))
                            }
                            (
                                s @ (State::DecInt(_)
                                | State::BinInt(_)
                                | State::OctInt(_)
                                | State::HexInt(_)
                                | State::Decimal(_)),
                                '_',
                            ) => s,
                            (s, _) => break state = s,
                        };
                        iter.next();
                    }
                    match state {
                        State::Dot => token::Token::Dot,
                        State::Zero => token::Token::DecInt("0".to_string()),
                        State::DecInt(s) => token::Token::DecInt(s),
                        State::BinInt(s) => token::Token::BinInt(s),
                        State::OctInt(s) => token::Token::OctInt(s),
                        State::HexInt(s) => token::Token::HexInt(s),
                        State::Decimal(s) | State::Sci(s) => token::Token::Float(s),
                        _ => return Err(Error::InvalidNumericLiteral(range_gen(iter.peek()))),
                    }
                }
                '+' => {
                    if iter.next_if(|&(_, c)| c == '+').is_some() {
                        token::Token::DoublePlus
                    } else if iter.next_if(|&(_, c)| c == '=').is_some() {
                        token::Token::PlusEqual
                    } else {
                        token::Token::Plus
                    }
                }
                '-' => {
                    if iter.next_if(|&(_, c)| c == '-').is_some() {
                        token::Token::DoubleHyphen
                    } else if iter.next_if(|&(_, c)| c == '=').is_some() {
                        token::Token::HyphenEqual
                    } else {
                        token::Token::Hyphen
                    }
                }
                _ => return Err(Error::UnexpectedCharacter(pos::Start::new(line_num, start))),
            };
            tokens.push_back((range_gen(iter.peek()), token));
        }
    }
}
