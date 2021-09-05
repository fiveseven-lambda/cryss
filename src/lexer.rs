//! 字句解析．入力を読んで `token::Token` に分割
//!
//! 使用時： `use lexer::Lexer`

use crate::error::Error;
use crate::pos;
use crate::token;

mod inner;
use inner::Inner;

mod test;

use std::collections::VecDeque;
use std::io::BufRead;

pub struct Lexer {
    /// 標準入力，ファイル入力どちらも可
    reader: Box<dyn BufRead>,
    /// プロンプト文字 `> ` を出力するか否か
    prompt: bool,
    inner: Inner,
    /// トークンの入っているキュー
    queue: VecDeque<(pos::Range, token::Token)>,
}

impl Lexer {
    pub fn new(reader: Box<dyn BufRead>, prompt: bool) -> Lexer {
        Lexer {
            reader,
            prompt,
            inner: Inner::new(),
            queue: VecDeque::new(),
        }
    }
    /// 次の行を読んで， `inner` に渡す．
    ///
    /// ファイル終端に達していれば `Ok(false)`，
    /// そうでなければ `Ok(true)` を返す．
    fn read(&mut self, log: &mut Vec<String>) -> Result<bool, Error> {
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
            // 1 文字以上読んだ（まだファイル終端ではない）
            let result = self.inner.run(log.len(), &line, &mut self.queue);
            // たとえ result がエラーだったとしても `log` に push はする
            log.push(line);
            // ファイル終端でないので成功なら Ok(true) を返す
            result.map(|()| true)
        } else {
            // ファイルの終端に達した
            if let Some(pos) = self.inner.comment.pop() {
                Err(Error::UnterminatedComment(pos))
            } else if let Some((pos, _)) = self.inner.string.take() {
                Err(Error::UnterminatedStringLiteral(pos))
            } else {
                Ok(false)
            }
        }
    }
    /// 次のトークンを返す．
    ///
    /// ファイル終端に達していれば `Ok(None)` を返す．
    pub fn next(
        &mut self,
        log: &mut Vec<String>,
    ) -> Result<Option<(pos::Range, token::Token)>, Error> {
        Ok(loop {
            match self.queue.pop_front() {
                Some(token) => break Some(token),
                None => {
                    if !self.read(log)? {
                        break None;
                    }
                }
            }
        })
    }
    /// 次のトークンが `cond` を満たすならそれを返す．
    ///
    /// 次のいずれかのときに `Ok(None)` を返す．
    /// - ファイル終端に達している．
    /// - 次のトークンが `cond` を満たさない．
    pub fn next_if(
        &mut self,
        cond: impl FnOnce(&token::Token) -> bool,
        log: &mut Vec<String>,
    ) -> Result<Option<(pos::Range, token::Token)>, Error> {
        Ok(loop {
            match self.queue.front() {
                Some((_, token)) => break cond(token).then(|| self.queue.pop_front()).flatten(),
                None => {
                    if !self.read(log)? {
                        break None;
                    }
                }
            }
        })
    }
}
