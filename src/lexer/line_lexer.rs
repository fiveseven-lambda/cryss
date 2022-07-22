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
            match first {
                'a'..='z' | 'A'..='Z' | '_' | '$' => {
                    let mut s = first.to_string();
                    let end = loop {
                        match iter.peek() {
                            Some(&(_, c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$'))) => {
                                s.push(c)
                            }
                            Some(&(i, _)) => break Some(i),
                            None => break None,
                        }
                        iter.next();
                    };
                    tokens.push_back((
                        pos::Range::from_line_byte(line_num, start, end),
                        token::Token::Identifier(s),
                    ));
                }
                '0'..='9' | '.' => {
                    enum State {
                        Zero,
                        BinIntIncomplete,
                        BinInt(String),
                        BinIntSuffix(String, String),
                        OctIntIncomplete,
                        OctInt(String),
                        OctIntSuffix(String, String),
                        DecInt(String),
                        DecIntSuffix(String, String),
                        HexIntIncomplete,
                        HexInt(String),
                        HexIntSuffix(String, String),
                        Dot,
                        Decimal(String),
                        ScientificIncomplete(String),
                        ScientificIncompleteSign(String),
                        Scientific(String),
                        FloatSuffix(String, String),
                    }

                    let mut state = match first {
                        '.' => State::Dot,
                        '0' => State::Zero,
                        _ => State::DecInt(first.into()),
                    };
                    loop {
                        state = match (state, iter.peek()) {
                            (State::Dot, Some(&(_, c @ '0'..='9'))) => {
                                State::Decimal(format!(".{c}"))
                            }
                            (State::Zero, Some(&(_, c @ '0'..='9'))) => {
                                State::DecInt(c.to_string())
                            }
                            (State::Zero, Some(&(_, 'b'))) => State::BinIntIncomplete,
                            (State::Zero, Some(&(_, 'o'))) => State::OctIntIncomplete,
                            (State::Zero, Some(&(_, 'x'))) => State::HexIntIncomplete,
                            (State::Zero, Some(&(_, '.'))) => State::Decimal(".".to_string()),
                            (State::BinIntIncomplete, Some(&(_, c @ '0'..='9'))) => {
                                State::BinInt(c.to_string())
                            }
                            (State::OctIntIncomplete, Some(&(_, c @ '0'..='9'))) => {
                                State::OctInt(c.to_string())
                            }
                            (
                                State::HexIntIncomplete,
                                Some(&(_, c @ ('0'..='9' | 'a'..='f' | 'A'..='F'))),
                            ) => State::HexInt(c.to_string()),
                            (State::BinInt(mut s), Some(&(_, c @ '0'..='9'))) => {
                                s.push(c);
                                State::BinInt(s)
                            }
                            (State::OctInt(mut s), Some(&(_, c @ '0'..='9'))) => {
                                s.push(c);
                                State::OctInt(s)
                            }
                            (State::DecInt(mut s), Some(&(_, c @ '0'..='9'))) => {
                                s.push(c);
                                State::DecInt(s)
                            }
                            (State::DecInt(mut s), Some(&(_, '.'))) => {
                                s.push('.');
                                State::Decimal(s)
                            }
                            (
                                State::HexInt(mut s),
                                Some(&(_, c @ ('0'..='9' | 'a'..='f' | 'A'..='F'))),
                            ) => {
                                s.push(c);
                                State::HexInt(s)
                            }
                            (State::Decimal(mut s), Some(&(_, c @ ('0'..='9')))) => {
                                s.push(c);
                                State::Decimal(s)
                            }
                            (State::Zero, Some(&(_, c @ ('e' | 'E')))) => {
                                State::ScientificIncomplete(format!("0{c}"))
                            }
                            (
                                State::DecInt(mut s) | State::Decimal(mut s),
                                Some(&(_, c @ ('e' | 'E'))),
                            ) => {
                                s.push(c);
                                State::ScientificIncomplete(s)
                            }
                            (State::ScientificIncomplete(mut s), Some(&(_, c @ ('+' | '-')))) => {
                                s.push(c);
                                State::ScientificIncompleteSign(s)
                            }
                            (
                                State::Scientific(mut s)
                                | State::ScientificIncomplete(mut s)
                                | State::ScientificIncompleteSign(mut s),
                                Some(&(_, c @ ('0'..='9'))),
                            ) => {
                                s.push(c);
                                State::Scientific(s)
                            }
                            (State::Zero, Some(&(_, '_'))) => State::DecInt("0".to_string()),
                            (
                                prev @ (State::BinInt(_)
                                | State::OctInt(_)
                                | State::DecInt(_)
                                | State::HexInt(_)
                                | State::Decimal(_)
                                | State::Scientific(_)
                                | State::ScientificIncomplete(_)
                                | State::ScientificIncompleteSign(_)),
                                Some(&(_, '_')),
                            ) => prev,
                            (State::Zero, Some(&(_, c @ ('a'..='z' | 'A'..='Z')))) => {
                                State::DecIntSuffix("0".to_string(), c.to_string())
                            }
                            (State::BinInt(s), Some(&(_, c @ ('a'..='z' | 'A'..='Z')))) => {
                                State::BinIntSuffix(s, c.to_string())
                            }
                            (State::OctInt(s), Some(&(_, c @ ('a'..='z' | 'A'..='Z')))) => {
                                State::OctIntSuffix(s, c.to_string())
                            }
                            (State::DecInt(s), Some(&(_, c @ ('a'..='z' | 'A'..='Z')))) => {
                                State::DecIntSuffix(s, c.to_string())
                            }
                            (State::HexInt(s), Some(&(_, c @ ('g'..='z' | 'G'..='Z')))) => {
                                State::HexIntSuffix(s, c.to_string())
                            }
                            (
                                State::Decimal(s) | State::Scientific(s),
                                Some(&(_, c @ ('a'..='z' | 'A'..='Z'))),
                            ) => State::FloatSuffix(s, c.to_string()),
                            (
                                State::BinIntSuffix(s, mut suffix),
                                Some(&(_, c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$'))),
                            ) => {
                                suffix.push(c);
                                State::BinIntSuffix(s, suffix)
                            }
                            (
                                State::OctIntSuffix(s, mut suffix),
                                Some(&(_, c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$'))),
                            ) => {
                                suffix.push(c);
                                State::OctIntSuffix(s, suffix)
                            }
                            (
                                State::DecIntSuffix(s, mut suffix),
                                Some(&(_, c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$'))),
                            ) => {
                                suffix.push(c);
                                State::DecIntSuffix(s, suffix)
                            }
                            (
                                State::HexIntSuffix(s, mut suffix),
                                Some(&(_, c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$'))),
                            ) => {
                                suffix.push(c);
                                State::HexIntSuffix(s, suffix)
                            }
                            (
                                State::FloatSuffix(s, mut suffix),
                                Some(&(_, c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$'))),
                            ) => {
                                suffix.push(c);
                                State::FloatSuffix(s, suffix)
                            }
                            (state, end) => {
                                let end = end.map(|&(i, _)| i);
                                let range = pos::Range::from_line_byte(line_num, start, end);
                                let token = match state {
                                    State::Zero => token::Token::DecInt("0".to_string()),
                                    State::BinInt(s) => token::Token::BinInt(s),
                                    State::BinIntSuffix(s, suffix) => {
                                        token::Token::BinIntSuffix(s, suffix)
                                    }
                                    State::DecInt(s) => token::Token::DecInt(s),
                                    State::DecIntSuffix(s, suffix) => {
                                        token::Token::DecIntSuffix(s, suffix)
                                    }
                                    State::OctInt(s) => token::Token::OctInt(s),
                                    State::OctIntSuffix(s, suffix) => {
                                        token::Token::OctIntSuffix(s, suffix)
                                    }
                                    State::HexInt(s) => token::Token::HexInt(s),
                                    State::HexIntSuffix(s, suffix) => {
                                        token::Token::HexIntSuffix(s, suffix)
                                    }
                                    State::Decimal(s) => token::Token::Float(s),
                                    State::Dot => token::Token::Dot,
                                    State::Scientific(s) => token::Token::Float(s),
                                    State::FloatSuffix(s, suffix) => {
                                        token::Token::FloatSuffix(s, suffix)
                                    }
                                    _ => return Err(Error::InvalidNumericLiteral(range)),
                                };
                                tokens.push_back((range, token));
                                break;
                            }
                        };
                        iter.next();
                    }
                }
                '+' => {
                    let range =
                        pos::Range::from_line_byte(line_num, start, iter.peek().map(|&(i, _)| i));
                    tokens.push_back((range, token::Token::Plus));
                }
                '-' => {
                    let range =
                        pos::Range::from_line_byte(line_num, start, iter.peek().map(|&(i, _)| i));
                    tokens.push_back((range, token::Token::Hyphen));
                }
                _ => return Err(Error::UnexpectedCharacter(pos::Start::new(line_num, start))),
            }
        }
    }
}
