//! ソースコードを読み，トークンに分割する．

use crate::{error, pos, token};
use std::collections::VecDeque;

/// 文字列をトークンに分割する．
///
/// この構造体は：
/// - 入出力をしない．
/// - 受け取った入力を所有しない．
/// - トークンを所有しない．
struct Inner {
    /// これが空でないなら，ブロックコメントの途中
    comment: Vec<pos::Pos>,
    /// これが Some なら，文字列リテラルの途中
    string: Option<(pos::Pos, String)>,
}

impl Inner {
    fn new() -> Inner {
        Inner {
            string: None,
            comment: Vec::new(),
        }
    }
    /// 一行（ `line` ）受け取って， `queue` にトークンを push する．
    /// `line_num` は今何行目か
    ///
    /// 基本的にはオートマトン：状態 × 文字 → 状態
    ///
    /// 状態の遷移を，
    /// そこでトークンが区切れるものとそうでないものに分ける．
    /// `match` でまず後者を処理し，
    /// default ケースとして前者を処理する（ `queue` への push はここだけで行う）．
    ///
    /// トークンが区切れないとき：次の状態を代入する．
    ///
    /// トークンが区切れるとき，新しいトークンが始まるとき：前のトークンを `queue` に push する．
    ///
    /// ファイルの末尾以外では，行は必ず `\n` で終わる（ `std::io::BufRead::read_line` の仕様）．
    /// ファイルの末尾は `\n` で終わっていなければならない．
    /// もしトークンの途中でファイルが終了したらエラーを返す
    fn run(
        &mut self,
        line_num: usize,
        line: &str,
        queue: &mut VecDeque<(pos::Range, token::Token)>,
    ) -> Result<(), error::Error> {
        let mut iter = line.char_indices().peekable();
        let mut prev = None;
        while let Some((index, c)) = iter.next() {
            let pos = pos::Pos::new(line_num, index);
            if self.comment.len() > 0 {
                // 今はブロックコメントの途中．
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
                    // 文字列の終わり．
                    // 次のループで queue に push してもらう
                    prev = Some((start, State::String(string)));
                    continue;
                }
            } else {
                if let Some((_, string)) = &mut self.string {
                    // 文字列の途中．
                    string.push(match c {
                        '\\' => match iter
                            .next()
                            .ok_or(error::Error::NoCharacterAfterBackSlash(pos))?
                            .1
                        {
                            // エスケープ
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            '0' => '\0',
                            // バックスラッシュの直後の文字を push
                            // `"` や `'` のエスケープを含む
                            c => c,
                        },
                        c => c,
                    });
                    continue;
                }
            }
            prev = match prev {
                Some((start, prev_state)) => {
                    let next_state = match (prev_state, c) {
                        (State::Identifier, 'a'..='z' | 'A'..='Z' | '_' | '$' | '0'..='9') => {
                            State::Identifier
                        }
                        (State::Parameter, 'a'..='z' | 'A'..='Z' | '_' | '$' | '0'..='9') => {
                            State::Parameter
                        }
                        (State::Integer, '0'..='9') => State::Integer,
                        (State::Integer, '.') => State::Decimal,
                        (State::Dot | State::Decimal, '0'..='9') => State::Decimal,
                        (State::Integer | State::Decimal, 'e' | 'E') => State::ScientificIncomplete,
                        (State::ScientificIncomplete, '+' | '-') => State::ScientificSign,
                        (
                            State::ScientificIncomplete | State::ScientificSign | State::Scientific,
                            '0'..='9',
                        ) => State::Scientific,
                        (State::Equal, '=') => State::DoubleEqual,
                        (State::Exclamation, '=') => State::ExclamationEqual,
                        (State::Ampersand, '&') => State::DoubleAmpersand,
                        (State::Bar, '|') => State::DoubleBar,
                        (State::Less, '<') => State::DoubleLess,
                        (State::Greater, '>') => State::DoubleGreater,
                        (State::Slash, '/') => {
                            // この行はこれ以降ラインコメント．
                            // `/` の直前のトークンは push 済みなので
                            // return してよい．
                            return Ok(());
                        }
                        (State::Slash, '*') => {
                            // ブロックコメントが，今始まる．
                            // `/` の直前のトークンは push 済み．
                            self.comment.push(start);
                            // prev は今所有権を失っているので，
                            // None を代入しておく．
                            prev = None;
                            continue;
                        }
                        (prev_state, c) => {
                            // トークンが区切れた．
                            let token = match prev_state {
                                State::Identifier => match &line[start.byte()..index] {
                                    "real" => token::Token::KeywordReal,
                                    "boolean" => token::Token::KeywordBoolean,
                                    "Sound" => token::Token::KeywordSound,
                                    "string" => token::Token::KeywordString,
                                    "if" => token::Token::KeywordIf,
                                    "while" => token::Token::KeywordWhile,
                                    "for" => token::Token::KeywordFor,
                                    "let" => token::Token::KeywordLet,
                                    "break" => token::Token::KeywordBreak,
                                    "continue" => token::Token::KeywordContinue,
                                    s => token::Token::Identifier(s.to_string()),
                                },
                                State::Parameter => {
                                    token::Token::Parameter(line[start.byte()..index].to_string())
                                }
                                State::Integer | State::Decimal | State::Scientific => {
                                    match line[start.byte()..index].parse() {
                                        Ok(value) => token::Token::Number(value),
                                        Err(err) => {
                                            return Err(error::Error::ParseFloatError(
                                                pos::Range::new(start, pos),
                                                err,
                                            ))
                                        }
                                    }
                                }
                                State::ScientificIncomplete | State::ScientificSign => {
                                    return Err(error::Error::IncompleteScientificNotation(
                                        pos::Range::new(start, pos),
                                    ));
                                }
                                State::String(string) => token::Token::String(string),
                                State::Plus => token::Token::Plus,
                                State::Minus => token::Token::Minus,
                                State::Asterisk => token::Token::Asterisk,
                                State::Slash => token::Token::Slash,
                                State::Percent => token::Token::Percent,
                                State::Circumflex => token::Token::Circumflex,
                                State::Equal => token::Token::Equal,
                                State::DoubleEqual => token::Token::DoubleEqual,
                                State::Exclamation => token::Token::Exclamation,
                                State::ExclamationEqual => token::Token::ExclamationEqual,
                                State::Less => token::Token::Less,
                                State::DoubleLess => token::Token::DoubleLess,
                                State::Greater => token::Token::Greater,
                                State::DoubleGreater => token::Token::DoubleGreater,
                                State::DoubleAmpersand => token::Token::DoubleAmpersand,
                                State::Bar => token::Token::Bar,
                                State::DoubleBar => token::Token::DoubleBar,
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
                                State::Ampersand => {
                                    return Err(error::Error::SingleAmpersand(pos::Range::new(
                                        start, pos,
                                    )))
                                }
                                State::Dot => {
                                    return Err(error::Error::SingleDot(pos::Range::new(
                                        start, pos,
                                    )))
                                }
                            };
                            // queue への push_back を行うのはここ 1 箇所だけ．
                            queue.push_back((pos::Range::new(start, pos.clone()), token));
                            // あとは None からの遷移と同じ
                            prev = self.begin(pos, c)?;
                            continue;
                        }
                    };
                    Some((start, next_state))
                }
                None => self.begin(pos, c)?,
            };
        }
        if prev.is_some() {
            Err(error::Error::NoLineFeedAtEOF)
        } else {
            Ok(())
        }
    }
    /// None からの遷移
    fn begin(&mut self, pos: pos::Pos, c: char) -> Result<Option<(pos::Pos, State)>, error::Error> {
        let state = match c {
            'a'..='z' | 'A'..='Z' | '_' => State::Identifier,
            '$' => State::Parameter,
            '0'..='9' => State::Integer,
            '"' => {
                // self.string が None でなくなることで，オートマトンの遷移から抜ける
                self.string = Some((pos, String::new()));
                // 文字列リテラルの終了後に None が入っているように
                return Ok(None);
            }
            '+' => State::Plus,
            '-' => State::Minus,
            '*' => State::Asterisk,
            '/' => State::Slash,
            '%' => State::Percent,
            '^' => State::Circumflex,
            '=' => State::Equal,
            '!' => State::Exclamation,
            '<' => State::Less,
            '>' => State::Greater,
            '&' => State::Ampersand,
            '|' => State::Bar,
            ':' => State::Colon,
            ';' => State::Semicolon,
            ',' => State::Comma,
            '.' => State::Dot,
            '?' => State::Question,
            '(' => State::OpeningParenthesis,
            ')' => State::ClosingParenthesis,
            '[' => State::OpeningBracket,
            ']' => State::ClosingBracket,
            '{' => State::OpeningBrace,
            '}' => State::ClosingBrace,
            _ if c.is_ascii_whitespace() => return Ok(None),
            _ => return Err(error::Error::UnexpectedCharacter(pos)),
        };
        Ok(Some((pos, state)))
    }
}

/// オートマトンの状態
///
/// 実際に `Inner::run()` が状態として持つのは `Option<(pos::Pos, State)>`
/// - `None` : トークンではない（空白）
/// - `Some(start, state)` : `start` がトークンの開始位置
enum State {
    /// 識別子．
    /// - None + [`a`-`z` `A`-`Z` `_`] -> `Identifier`
    /// - `Identifier` + [`a`-`z` `A`-`Z` `_` `$` `0`-`9`] -> `Identifier`
    Identifier,
    /// 属性．
    /// - None + `$` -> `Parameter`
    /// - `Parameter` + [`a`-`z` `A`-`Z` `_` `$` `0`-`9`] -> `Parameter`
    Parameter,
    /// 数値リテラル．
    /// - None + [`0`-`9`] -> `Integer`
    /// - `Integer` + [`0`-`9`] -> `Integer`
    Integer,
    /// 小数点を含む数値リテラル．
    /// - `Integer` + `.` -> `Decimal`
    /// - `Dot` + [`0`-`9`] -> `Decimal`
    /// - `Decimal` + [`0`-`9`] -> `Decimal`
    Decimal,
    /// 指数表記の途中（ e まで）
    /// - `Integer` + [`e` `E`] -> `ScientificIncomplete`
    /// - `Decimal` + [`e` `E`] -> `ScientificIncomplete`
    ScientificIncomplete,
    /// 指数表記の途中（指数部分の符号まで）
    /// - `ScientificIncomplete` + [`+` `-`] -> `ScientificSign`
    ScientificSign,
    /// 指数表記の数値リテラル
    /// - `ScientificIncomplete` + [`0`-`9`] -> `Scientific`
    /// - `ScientificSign` + [`0`-`9`] -> `Scientific`
    /// - `Scientific` + [`0`-`9`] -> `Scientific`
    Scientific,
    /// 文字列リテラル．
    /// ただしオートマトンには含まれない
    String(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Circumflex,
    Equal,
    DoubleEqual,
    Exclamation,
    ExclamationEqual,
    Less,
    DoubleLess,
    Greater,
    DoubleGreater,
    /// 単独の `&`
    Ampersand,
    DoubleAmpersand,
    Bar,
    DoubleBar,
    Colon,
    Semicolon,
    Comma,
    /// 単独の `.`
    Dot,
    Question,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    OpeningBrace,
    ClosingBrace,
}

/// 内部で `Inner::run()` を呼び出す
pub struct Lexer<BufRead> {
    /// 標準入力，ファイル入力どちらも可
    reader: BufRead,
    /// プロンプト文字 `> ` を出力するか否か
    prompt: bool,
    inner: Inner,
    /// トークンの入っているキュー
    queue: VecDeque<(pos::Range, token::Token)>,
}

impl<BufRead> Lexer<BufRead> {
    pub fn new(reader: BufRead, prompt: bool) -> Lexer<BufRead> {
        Lexer {
            reader,
            prompt,
            inner: Inner::new(),
            queue: VecDeque::new(),
        }
    }
}

impl<BufRead: std::io::BufRead> Lexer<BufRead> {
    /// 次のトークンを返す．
    ///
    /// 必要なだけ次の行を読み，
    /// 読んだ行は（字句解析が成功したか失敗したかに関わらず）ログに格納する．
    ///
    /// - 字句解析に失敗したら，エラーを返す．
    /// - 字句解析に成功したら， `Option` に包んでトークンを返す
    ///   （ `None` は，ファイル終端に達し全てのトークンを読み切ったことを意味する）．
    pub fn next(
        &mut self,
        log: &mut Vec<String>,
    ) -> Result<Option<(pos::Range, token::Token)>, error::Error> {
        loop {
            match self.queue.pop_front() {
                Some(token) => return Ok(Some(token)),
                None => {
                    let mut line = String::new();
                    if self.prompt {
                        // 対話環境ではプロンプトを出す
                        // ファイルから読むときは出さない
                        use std::io::Write;
                        print!("> ");
                        std::io::stdout().flush().expect("failed to flush stdout");
                    }
                    if self
                        .reader
                        .read_line(&mut line)
                        .expect("failed to read input")
                        > 0
                    {
                        let result = self.inner.run(log.len(), &line, &mut self.queue);
                        log.push(line);
                        result?;
                    } else {
                        // ファイルの末尾に達した．
                        return if let Some(pos) = self.inner.comment.pop() {
                            Err(error::Error::UnterminatedComment(pos))
                        } else if let Some((pos, _)) = self.inner.string.take() {
                            Err(error::Error::UnterminatedStringLiteral(pos))
                        } else {
                            Ok(None)
                        };
                    }
                }
            }
        }
    }
}
