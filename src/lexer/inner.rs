use crate::error::Error;
use crate::pos;
use crate::token;
use std::collections::VecDeque;

/// Lexer が内部で用いる．
///
/// この構造体は：
/// - 入出力をしない．
/// - 受け取った入力を所有しない．
/// - トークンを格納するコンテナを所有しない．
pub struct Inner {
    /// これが空でないなら，ブロックコメントの途中
    pub comment: Vec<pos::Pos>,
    /// これが Some なら，文字列リテラルの途中
    pub string: Option<(pos::Pos, String)>,
}

impl Inner {
    pub fn new() -> Inner {
        Inner {
            string: None,
            comment: Vec::new(),
        }
    }
    /// 行 `line: &str` を受け取って，
    /// `queue` にトークンを push する．
    /// トークンに位置情報を与えるために
    /// 今何行目か知る必要があり，これは
    /// 引数 `line_num` で受け取る．
    ///
    /// オートマトンを用いて字句解析を行う．
    /// 遷移：状態×文字→状態
    ///
    /// 状態の遷移を，そこで新しいトークンが始まるものとそうでないものに分ける．
    /// `match` でまず後者を処理し，
    /// デフォルトケースとして前者を処理する
    ///
    /// 新しいトークンが始まるとき，前のトークンを `queue` に push する．
    /// ここ以外で `queue` に対する操作は行わない．
    pub fn run(
        &mut self,
        line_num: usize,
        line: &str,
        queue: &mut VecDeque<(pos::Range, token::Token)>,
    ) -> Result<(), Error> {
        // 行を 1 文字ずつ読んでいく
        let mut iter = line.char_indices().peekable();
        // オートマトンの状態．
        //
        // `Some((start, state))` ならトークンの途中で，
        // `start` がトークンの開始位置．
        //
        // `None` なら空白文字．
        let mut prev: Option<(pos::Pos, State)> = None;
        while let Some((index, c)) = iter.next() {
            // 今注目している文字の位置
            let pos = pos::Pos::new(line_num, index);

            if !self.comment.is_empty() {
                // ブロックコメントの途中．
                if c == '*' {
                    if let Some((_, '/')) = iter.peek() {
                        // コメントの終了．
                        // peek した `/` を読む．
                        iter.next();
                        self.comment.pop();
                    }
                } else if c == '/' {
                    match iter.peek() {
                        Some((_, '*')) => {
                            // コメントのネスト．
                            // peek した `*` を読む．
                            iter.next();
                            self.comment.push(pos);
                        }
                        Some((_, '/')) => {
                            // ブロックコメント内のラインコメント．
                            return Ok(());
                        }
                        _ => {}
                    }
                }
                continue;
            }
            if c == '"' {
                if let Some((start, string)) = self.string.take() {
                    // 文字列リテラルの終わり．
                    // 次のループで queue に push してもらう
                    prev = Some((start, State::String(string)));
                } else {
                    // 文字列リテラルの始まり．
                    self.string = Some((pos, String::new()));
                }
                continue;
            }
            if let Some((_, string)) = &mut self.string {
                // 文字列の途中．
                string.push(match c {
                    '\\' => match iter.next() {
                        Some((_, c)) => match c {
                            // エスケープ
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            '0' => '\0',
                            // バックスラッシュの直後の文字を push
                            // `"` や `'` のエスケープを含む
                            c => c,
                        },
                        None => return Err(Error::UnterminatedStringLiteral(pos)),
                    },
                    c => c,
                });
                continue;
            }

            // 状態の遷移
            prev = match prev {
                Some((start, prev_state)) => {
                    let next_state = match (prev_state, c) {
                        (State::Identifier, 'a'..='z' | 'A'..='Z' | '_' | '$' | '0'..='9') => {
                            State::Identifier
                        }
                        (State::Zero | State::Integer, '0'..='9') => State::Integer,
                        (State::Zero, 'b') => State::BinaryIncomplete,
                        (State::BinaryIncomplete | State::Binary, '0'..='1') => State::Binary,
                        (State::Zero, 'o') => State::OctalIncomplete,
                        (State::OctalIncomplete | State::Octal, '0'..='7') => State::Octal,
                        (State::Zero, 'x') => State::HexadecimalIncomplete,
                        (
                            State::HexadecimalIncomplete | State::Hexadecimal,
                            '0'..='9' | 'a'..='z' | 'A'..='Z',
                        ) => State::Hexadecimal,
                        (State::Dot | State::Decimal, '0'..='9') => State::Decimal,
                        (State::Zero | State::Integer, '.') => State::Decimal,
                        (State::Zero | State::Integer | State::Decimal, 'e' | 'E') => {
                            State::ScientificIncomplete
                        }
                        (State::ScientificIncomplete, '+' | '-') => State::ScientificSign,
                        (
                            State::ScientificIncomplete | State::ScientificSign | State::Scientific,
                            '0'..='9',
                        ) => State::Scientific,
                        (State::Asterisk, '*') => State::DoubleAsterisk,
                        (State::Slash, '/') => {
                            // この行はこれ以降ラインコメント．
                            // `/` の直前のトークンは push 済みなので
                            // return してよい．
                            return Ok(());
                        }
                        (State::Slash, '*') => {
                            // ブロックコメントの始まり．
                            // `/` の直前のトークンは push 済み．
                            self.comment.push(start);
                            // コメント終了後は空白後と同様に
                            // 新しいトークンが始まるので，
                            // None を代入．
                            prev = None;
                            continue;
                        }
                        (State::Equal, '=') => State::DoubleEqual,
                        (State::Exclamation, '=') => State::ExclamationEqual,
                        (State::Less, '=') => State::LessEqual,
                        (State::Less, '<') => State::DoubleLess,
                        (State::DoubleLess, '<') => State::TripleLess,
                        (State::Greater, '=') => State::GreaterEqual,
                        (State::Greater, '>') => State::DoubleGreater,
                        (State::DoubleGreater, '>') => State::TripleGreater,
                        (State::Ampersand, '&') => State::DoubleAmpersand,
                        (State::Bar, '|') => State::DoubleBar,
                        (state, c) => {
                            // 新しいトークンの開始
                            let s = &line[start.byte()..index];
                            let range = pos::Range::new(start, pos.clone());
                            let token = match state.accept(s) {
                                Ok(token) => token,
                                Err(()) => return Err(Error::InvalidToken(range)),
                            };
                            queue.push_back((range, token));
                            prev = State::begin(c, pos)?;
                            // 次の文字を読みに行く．
                            continue;
                        }
                    };
                    // 新しいトークンが始まらなかった
                    //（新しいトークンが始まると continue でここは飛ばされるので）．
                    (start, next_state).into()
                }
                None => State::begin(c, pos)?,
            }
        }
        // 行末のトークンを読む
        if let Some((start, state)) = prev {
            let s = &line[start.byte()..];
            let range = pos::Range::new(start, pos::Pos::new(line_num, line.len()));
            let token = match state.accept(s) {
                Ok(token) => token,
                Err(()) => return Err(Error::InvalidToken(range)),
            };
            queue.push_back((range, token));
        }
        Ok(())
    }
}

/// オートマトンの状態
///
/// 実際に `Inner::run()` が状態として持つのは `Option<(pos::Pos, State)>`
/// - `None` : トークンではない（空白）
/// - `Some(start, state)` : `start` がトークンの開始位置
enum State {
    /// 識別子
    Identifier,
    /// ゼロ
    Zero,
    /// 10 進整数
    Integer,
    /// `0b`（2 進数の開始）
    BinaryIncomplete,
    /// 2 進数
    Binary,
    /// `0o`（8 進数の開始）
    OctalIncomplete,
    /// 8 進数
    Octal,
    /// `0x`（16 進数の開始）
    HexadecimalIncomplete,
    /// 16 進数
    Hexadecimal,
    /// 小数
    Decimal,
    /// 指数表記の `e` `E` まで
    ScientificIncomplete,
    /// 指数表記の `+` `-` まで
    ScientificSign,
    /// 指数表記
    Scientific,
    /// 文字列リテラル
    String(String),
    Plus,
    Hyphen,
    Asterisk,
    DoubleAsterisk,
    Slash,
    Percent,
    Equal,
    DoubleEqual,
    Exclamation,
    ExclamationEqual,
    Less,
    LessEqual,
    DoubleLess,
    TripleLess,
    Greater,
    GreaterEqual,
    DoubleGreater,
    TripleGreater,
    Ampersand,
    DoubleAmpersand,
    Bar,
    DoubleBar,
    Circumflex,
    Dot,
    Colon,
    Semicolon,
    Comma,
    Question,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    OpeningBrace,
    ClosingBrace,
}

impl State {
    /// 状態 None からの遷移先
    fn begin(c: char, pos: pos::Pos) -> Result<Option<(pos::Pos, State)>, Error> {
        let state = match c {
            'a'..='z' | 'A'..='Z' | '_' | '$' => State::Identifier,
            '0' => State::Zero,
            '1'..='9' => State::Integer,
            '+' => State::Plus,
            '-' => State::Hyphen,
            '*' => State::Asterisk,
            '/' => State::Slash,
            '%' => State::Percent,
            '=' => State::Equal,
            '!' => State::Exclamation,
            '<' => State::Less,
            '>' => State::Greater,
            '&' => State::Ampersand,
            '|' => State::Bar,
            '^' => State::Circumflex,
            '.' => State::Dot,
            ':' => State::Colon,
            ';' => State::Semicolon,
            ',' => State::Comma,
            '?' => State::Question,
            '(' => State::OpeningParenthesis,
            ')' => State::ClosingParenthesis,
            '[' => State::OpeningBracket,
            ']' => State::ClosingBracket,
            '{' => State::OpeningBrace,
            '}' => State::ClosingBrace,
            _ if c.is_ascii_whitespace() => return Ok(None),
            _ => return Err(Error::UnexpectedCharacter(pos)),
        };
        Ok(Some((pos, state)))
    }
    /// 状態の受容
    fn accept(self, s: &str) -> Result<token::Token, ()> {
        Ok(match self {
            State::Identifier => token::Token::Identifier(s.to_string()),
            State::Zero | State::Integer | State::Binary | State::Octal | State::Hexadecimal => {
                token::Token::Integer(s.to_string())
            }
            State::String(string) => token::Token::String(string),
            State::Decimal | State::Scientific => token::Token::Real(s.to_string()),
            State::Plus => token::Token::Plus,
            State::Hyphen => token::Token::Hyphen,
            State::Asterisk => token::Token::Asterisk,
            State::DoubleAsterisk => token::Token::DoubleAsterisk,
            State::Slash => token::Token::Slash,
            State::Percent => token::Token::Percent,
            State::Equal => token::Token::Equal,
            State::DoubleEqual => token::Token::DoubleEqual,
            State::Exclamation => token::Token::Exclamation,
            State::ExclamationEqual => token::Token::ExclamationEqual,
            State::Less => token::Token::Less,
            State::LessEqual => token::Token::LessEqual,
            State::DoubleLess => token::Token::DoubleLess,
            State::TripleLess => token::Token::TripleLess,
            State::Greater => token::Token::Greater,
            State::GreaterEqual => token::Token::GreaterEqual,
            State::DoubleGreater => token::Token::DoubleGreater,
            State::TripleGreater => token::Token::TripleGreater,
            State::Ampersand => token::Token::Ampersand,
            State::DoubleAmpersand => token::Token::DoubleAmpersand,
            State::Bar => token::Token::Bar,
            State::DoubleBar => token::Token::DoubleBar,
            State::Circumflex => token::Token::Circumflex,
            State::Dot => token::Token::Dot,
            State::Colon => token::Token::Colon,
            State::Semicolon => token::Token::Semicolon,
            State::Comma => token::Token::Comma,
            State::Question => token::Token::Question,
            State::OpeningParenthesis => token::Token::OpeningParenthesis,
            State::ClosingParenthesis => token::Token::ClosingParenthesis,
            State::OpeningBracket => token::Token::OpeningBracket,
            State::ClosingBracket => token::Token::ClosingBracket,
            State::OpeningBrace => token::Token::OpeningBrace,
            State::ClosingBrace => token::Token::ClosingBrace,
            State::BinaryIncomplete
            | State::OctalIncomplete
            | State::HexadecimalIncomplete
            | State::ScientificIncomplete
            | State::ScientificSign => return Err(()),
        })
    }
}
