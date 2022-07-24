use crate::error::Error;
use crate::pos;
use crate::token;
use std::collections::VecDeque;

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
            Err(Error::UnterminatedComment(std::mem::take(
                &mut self.comment,
            )))
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
        tokens: &mut VecDeque<token::RToken>,
    ) -> Result<(), Error> {
        let mut iter = line.char_indices().peekable();
        while let Some((index, ch)) = iter.next() {
            let second_is = |ch1| move |&(_, ch2): &(usize, char)| ch1 == ch2;
            let range_gen = |peeked: Option<&(usize, char)>| {
                pos::Range::from_line_byte(line_num, index, peeked.map(|&(i, _)| i))
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
                    tokens.push_back((range, token::Token::String(string)));
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
                            c => c,
                        },
                        None => {
                            // 文字列リテラルの開始場所を得る
                            let start = std::mem::replace(start, pos::Start::new(0, 0));
                            return Err(Error::UnterminatedStringLiteral(start));
                        }
                    },
                    c => c,
                });
            } else if !ch.is_ascii_whitespace() {
                let token = match ch {
                    'a'..='z' | 'A'..='Z' | '_' | '$' => {
                        let mut s = ch.to_string();
                        while let Some(&(_, ch @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$'))) =
                            iter.peek()
                        {
                            s.push(ch);
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
                        let mut state = match ch {
                            '.' => State::Dot,
                            '0' => State::Zero,
                            _ => State::DecInt(ch.into()),
                        };
                        loop {
                            fn append_char(mut s: String, ch: char) -> String {
                                s.push(ch);
                                s
                            }
                            let ch = match iter.peek() {
                                Some(&(_, ch)) => ch,
                                None => break,
                            };
                            state = match (state, ch) {
                                (State::Dot, '0'..='9') => State::Decimal(format!(".{ch}")),
                                (State::Zero, '_') => State::DecInt("0".to_string()),
                                (State::Zero, 'b') => State::BinInt(String::new()),
                                (State::Zero, 'o') => State::OctInt(String::new()),
                                (State::Zero, 'x') => State::HexInt(String::new()),
                                (State::Zero, '.') => State::Decimal("0.".to_string()),
                                (State::Zero, '0'..='9') => State::Decimal(ch.to_string()),
                                (State::DecInt(s), '.') => State::Decimal(append_char(s, '.')),
                                (State::Decimal(s), '0'..='9') => {
                                    State::Decimal(append_char(s, ch))
                                }
                                (State::BinInt(s), '0'..='9') => State::BinInt(append_char(s, ch)),
                                (State::OctInt(s), '0'..='9') => State::OctInt(append_char(s, ch)),
                                (State::DecInt(s), '0'..='9') => State::DecInt(append_char(s, ch)),
                                (State::HexInt(s), '0'..='9' | 'a'..='f' | 'A'..='F') => {
                                    State::HexInt(append_char(s, ch))
                                }
                                (State::Zero, 'e' | 'E') => State::SciE(format!("0{ch}")),
                                (State::DecInt(s) | State::Decimal(s), 'e' | 'E') => {
                                    State::SciE(append_char(s, ch))
                                }
                                (State::SciE(s), '+' | '-') => State::SciSign(append_char(s, ch)),
                                (State::SciE(s) | State::SciSign(s) | State::Sci(s), '0'..='9') => {
                                    State::Sci(append_char(s, ch))
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
                        if iter.next_if(second_is('+')).is_some() {
                            token::Token::DoublePlus
                        } else if iter.next_if(second_is('=')).is_some() {
                            token::Token::PlusEqual
                        } else {
                            token::Token::Plus
                        }
                    }
                    '-' => {
                        if iter.next_if(second_is('-')).is_some() {
                            token::Token::DoubleHyphen
                        } else if iter.next_if(second_is('=')).is_some() {
                            token::Token::HyphenEqual
                        } else {
                            token::Token::Hyphen
                        }
                    }
                    '*' => {
                        if iter.next_if(second_is('=')).is_some() {
                            token::Token::AsteriskEqual
                        } else {
                            token::Token::Asterisk
                        }
                    }
                    '/' => {
                        if iter.next_if(second_is('/')).is_some() {
                            return Ok(());
                        } else if iter.next_if(second_is('*')).is_some() {
                            self.comment.push(pos::Start::new(line_num, index));
                            continue;
                        } else if iter.next_if(second_is('=')).is_some() {
                            token::Token::SlashEqual
                        } else {
                            token::Token::Slash
                        }
                    }
                    '%' => {
                        if iter.next_if(second_is('=')).is_some() {
                            token::Token::PercentEqual
                        } else {
                            token::Token::Percent
                        }
                    }
                    '=' => {
                        if iter.next_if(second_is('=')).is_some() {
                            token::Token::DoubleEqual
                        } else {
                            token::Token::Equal
                        }
                    }
                    '!' => {
                        if iter.next_if(second_is('=')).is_some() {
                            token::Token::ExclamationEqual
                        } else {
                            token::Token::Exclamation
                        }
                    }
                    '<' => {
                        if iter.next_if(second_is('<')).is_some() {
                            if iter.next_if(second_is('<')).is_some() {
                                if iter.next_if(second_is('=')).is_some() {
                                    token::Token::TripleLessEqual
                                } else {
                                    token::Token::TripleLess
                                }
                            } else if iter.next_if(second_is('=')).is_some() {
                                token::Token::DoubleLessEqual
                            } else {
                                token::Token::DoubleLess
                            }
                        } else if iter.next_if(second_is('=')).is_some() {
                            token::Token::LessEqual
                        } else {
                            token::Token::Less
                        }
                    }
                    '>' => {
                        if iter.next_if(second_is('>')).is_some() {
                            if iter.next_if(second_is('>')).is_some() {
                                if iter.next_if(second_is('=')).is_some() {
                                    token::Token::TripleGreaterEqual
                                } else {
                                    token::Token::TripleGreater
                                }
                            } else if iter.next_if(second_is('=')).is_some() {
                                token::Token::DoubleGreaterEqual
                            } else {
                                token::Token::DoubleGreater
                            }
                        } else if iter.next_if(second_is('=')).is_some() {
                            token::Token::GreaterEqual
                        } else {
                            token::Token::Greater
                        }
                    }
                    '&' => {
                        if iter.next_if(second_is('&')).is_some() {
                            token::Token::DoubleAmpersand
                        } else if iter.next_if(second_is('=')).is_some() {
                            token::Token::AmpersandEqual
                        } else {
                            token::Token::Ampersand
                        }
                    }
                    '|' => {
                        if iter.next_if(second_is('|')).is_some() {
                            token::Token::DoubleBar
                        } else if iter.next_if(second_is('=')).is_some() {
                            token::Token::BarEqual
                        } else {
                            token::Token::Bar
                        }
                    }
                    '^' => {
                        if iter.next_if(second_is('=')).is_some() {
                            token::Token::CircumflexEqual
                        } else {
                            token::Token::Circumflex
                        }
                    }
                    ':' => token::Token::Colon,
                    ';' => token::Token::Semicolon,
                    ',' => token::Token::Comma,
                    '?' => token::Token::Question,
                    '(' => token::Token::OpeningParenthesis,
                    ')' => token::Token::ClosingParenthesis,
                    '[' => token::Token::OpeningBracket,
                    ']' => token::Token::ClosingBracket,
                    '{' => token::Token::OpeningBrace,
                    '}' => token::Token::ClosingBrace,
                    _ => return Err(Error::UnexpectedCharacter(pos::Start::new(line_num, index))),
                };
                tokens.push_back((range_gen(iter.peek()), token));
            }
        }
        Ok(())
    }
}
