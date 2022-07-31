use crate::error::Error;
use crate::pos;
use crate::token::{PToken, Token};
use std::collections::VecDeque;
use std::mem;

pub struct LineLexer {
    comment: Vec<pos::Start>,
    string: Option<(pos::Start, String)>,
}

impl LineLexer {
    pub fn new() -> LineLexer {
        LineLexer {
            comment: Vec::new(),
            string: None,
        }
    }

    pub fn deal_with_eof(&mut self) -> Result<(), Error> {
        if !self.comment.is_empty() {
            Err(Error::UnterminatedComment(mem::take(&mut self.comment)))
        } else if let Some((start, _)) = self.string.take() {
            Err(Error::UnterminatedStringLiteral(start))
        } else {
            Ok(())
        }
    }

    pub fn run(
        &mut self,
        line_num: usize,
        line: &str,
        tokens: &mut VecDeque<PToken>,
    ) -> Result<(), Error> {
        let mut iter = line.char_indices().peekable();
        while let Some((index, ch)) = iter.next() {
            let second_is = |ch1| move |&(_, ch2): &_| ch1 == ch2;
            let range_gen = |peeked: Option<_>| {
                pos::Range::new_single_line(line_num, index, peeked.map(|&(i, _)| i))
            };
            if !self.comment.is_empty() {
                if ch == '*' && iter.next_if(second_is('/')).is_some() {
                    self.comment.pop();
                } else if ch == '/' && iter.next_if(second_is('*')).is_some() {
                    self.comment.push(pos::Start::new(line_num, index));
                }
            } else if ch == '"' {
                if let Some((start, string)) = self.string.take() {
                    let end = pos::End::new(line_num, iter.peek().map(|&(i, _)| i));
                    let range = pos::Range::new(start, end);
                    tokens.push_back((range, Token::String(string)));
                } else {
                    let start = pos::Start::new(line_num, index);
                    self.string = Some((start, String::new()))
                }
            } else if let Some((start, string)) = &mut self.string {
                string.push(match ch {
                    '\\' => match iter.next() {
                        Some((_, ch2)) => match ch2 {
                            // エスケープ
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            '0' => '\0',
                            // バックスラッシュの直後の文字を push
                            // `"` や `'` のエスケープを含む
                            _ => ch2,
                        },
                        None => return Err(Error::UnterminatedStringLiteral(start.clone())),
                    },
                    _ => ch,
                });
            } else if !ch.is_ascii_whitespace() {
                // rename ch -> first_ch
                let first_ch = ch;
                #[allow(unused_variables)]
                let ch: ();
                // rename index -> first_index
                let first_index = index;
                #[allow(unused_variables)]
                let index: ();
                let token = match first_ch {
                    'a'..='z' | 'A'..='Z' | '_' | '$' => {
                        while iter
                            .next_if(|&(_, ch)| {
                                matches!(ch,
                                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$'
                                )
                            })
                            .is_some()
                        {}
                        let s = match iter.peek() {
                            Some(&(index, _)) => &line[first_index..index],
                            None => &line[first_index..],
                        };
                        match s {
                            "if" => Token::KeywordIf,
                            "for" => Token::KeywordFor,
                            "else" => Token::KeywordElse,
                            _ => Token::Identifier(s.to_owned()),
                        }
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
                        let mut state = match first_ch {
                            '.' => State::Dot,
                            '0' => State::Zero,
                            _ => State::DecInt(first_ch.into()),
                        };
                        while let Some(&(_, ch)) = iter.peek() {
                            let append_ch = |mut s: String| {
                                s.push(ch);
                                s
                            };
                            state = match (state, ch) {
                                (State::Dot, '0'..='9') => State::Decimal(format!(".{ch}")),
                                (State::Zero, '_') => State::DecInt("0".to_string()),
                                (State::Zero, 'b') => State::BinInt(String::new()),
                                (State::Zero, 'o') => State::OctInt(String::new()),
                                (State::Zero, 'x') => State::HexInt(String::new()),
                                (State::Zero, '.') => State::Decimal("0.".to_string()),
                                (State::Zero, '0'..='9') => State::Decimal(ch.to_string()),
                                (State::DecInt(s), '.') => State::Decimal(append_ch(s)),
                                (State::Decimal(s), '0'..='9') => State::Decimal(append_ch(s)),
                                (State::BinInt(s), '0'..='9') => State::BinInt(append_ch(s)),
                                (State::OctInt(s), '0'..='9') => State::OctInt(append_ch(s)),
                                (State::DecInt(s), '0'..='9') => State::DecInt(append_ch(s)),
                                (State::HexInt(s), '0'..='9' | 'a'..='f' | 'A'..='F') => {
                                    State::HexInt(append_ch(s))
                                }
                                (State::Zero, 'e' | 'E') => State::SciE(format!("0{ch}")),
                                (State::DecInt(s) | State::Decimal(s), 'e' | 'E') => {
                                    State::SciE(append_ch(s))
                                }
                                (State::SciE(s), '+' | '-') => State::SciSign(append_ch(s)),
                                (State::SciE(s) | State::SciSign(s) | State::Sci(s), '0'..='9') => {
                                    State::Sci(append_ch(s))
                                }
                                (
                                    s @ (State::DecInt(_)
                                    | State::BinInt(_)
                                    | State::OctInt(_)
                                    | State::HexInt(_)
                                    | State::Decimal(_)),
                                    '_',
                                ) => s,
                                (final_state, _) => {
                                    state = final_state;
                                    break;
                                }
                            };
                            iter.next();
                        }
                        match state {
                            State::Dot => Token::Dot,
                            State::Zero => Token::DecInt("0".to_string()),
                            State::DecInt(s) => Token::DecInt(s),
                            State::BinInt(s) => Token::BinInt(s),
                            State::OctInt(s) => Token::OctInt(s),
                            State::HexInt(s) => Token::HexInt(s),
                            State::Decimal(s) | State::Sci(s) => Token::Float(s),
                            _ => return Err(Error::InvalidNumericLiteral(range_gen(iter.peek()))),
                        }
                    }
                    '+' => {
                        if iter.next_if(second_is('+')).is_some() {
                            Token::DoublePlus
                        } else if iter.next_if(second_is('=')).is_some() {
                            Token::PlusEqual
                        } else {
                            Token::Plus
                        }
                    }
                    '-' => {
                        if iter.next_if(second_is('-')).is_some() {
                            Token::DoubleHyphen
                        } else if iter.next_if(second_is('=')).is_some() {
                            Token::HyphenEqual
                        } else {
                            Token::Hyphen
                        }
                    }
                    '*' => {
                        if iter.next_if(second_is('=')).is_some() {
                            Token::AsteriskEqual
                        } else {
                            Token::Asterisk
                        }
                    }
                    '/' => {
                        if iter.next_if(second_is('/')).is_some() {
                            return Ok(());
                        } else if iter.next_if(second_is('*')).is_some() {
                            self.comment.push(pos::Start::new(line_num, first_index));
                            continue;
                        } else if iter.next_if(second_is('=')).is_some() {
                            Token::SlashEqual
                        } else {
                            Token::Slash
                        }
                    }
                    '%' => {
                        if iter.next_if(second_is('=')).is_some() {
                            Token::PercentEqual
                        } else {
                            Token::Percent
                        }
                    }
                    '=' => {
                        if iter.next_if(second_is('=')).is_some() {
                            Token::DoubleEqual
                        } else {
                            Token::Equal
                        }
                    }
                    '!' => {
                        if iter.next_if(second_is('=')).is_some() {
                            Token::ExclamationEqual
                        } else {
                            Token::Exclamation
                        }
                    }
                    '<' => {
                        if iter.next_if(second_is('<')).is_some() {
                            if iter.next_if(second_is('<')).is_some() {
                                if iter.next_if(second_is('=')).is_some() {
                                    Token::TripleLessEqual
                                } else {
                                    Token::TripleLess
                                }
                            } else if iter.next_if(second_is('=')).is_some() {
                                Token::DoubleLessEqual
                            } else {
                                Token::DoubleLess
                            }
                        } else if iter.next_if(second_is('=')).is_some() {
                            Token::LessEqual
                        } else {
                            Token::Less
                        }
                    }
                    '>' => {
                        if iter.next_if(second_is('>')).is_some() {
                            if iter.next_if(second_is('>')).is_some() {
                                if iter.next_if(second_is('=')).is_some() {
                                    Token::TripleGreaterEqual
                                } else {
                                    Token::TripleGreater
                                }
                            } else if iter.next_if(second_is('=')).is_some() {
                                Token::DoubleGreaterEqual
                            } else {
                                Token::DoubleGreater
                            }
                        } else if iter.next_if(second_is('=')).is_some() {
                            Token::GreaterEqual
                        } else {
                            Token::Greater
                        }
                    }
                    '&' => {
                        if iter.next_if(second_is('&')).is_some() {
                            Token::DoubleAmpersand
                        } else if iter.next_if(second_is('=')).is_some() {
                            Token::AmpersandEqual
                        } else {
                            Token::Ampersand
                        }
                    }
                    '|' => {
                        if iter.next_if(second_is('|')).is_some() {
                            Token::DoubleBar
                        } else if iter.next_if(second_is('=')).is_some() {
                            Token::BarEqual
                        } else {
                            Token::Bar
                        }
                    }
                    '^' => {
                        if iter.next_if(second_is('=')).is_some() {
                            Token::CircumflexEqual
                        } else {
                            Token::Circumflex
                        }
                    }
                    ':' => Token::Colon,
                    ';' => Token::Semicolon,
                    ',' => Token::Comma,
                    '?' => Token::Question,
                    '#' => Token::Hash,
                    '~' => Token::Tilde,
                    '(' => Token::OpeningParenthesis,
                    ')' => Token::ClosingParenthesis,
                    '[' => Token::OpeningBracket,
                    ']' => Token::ClosingBracket,
                    '{' => Token::OpeningBrace,
                    '}' => Token::ClosingBrace,
                    _ => {
                        return Err(Error::UnexpectedCharacter(pos::Start::new(
                            line_num,
                            first_index,
                        )))
                    }
                };
                tokens.push_back((range_gen(iter.peek()), token));
            }
        }
        Ok(())
    }
}
