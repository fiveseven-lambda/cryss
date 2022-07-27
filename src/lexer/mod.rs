mod line_lexer;
use line_lexer::LineLexer;

mod test;

use std::collections::VecDeque;
use std::io::BufRead;

use crate::error::Error;
use crate::pos;
use crate::token::{PToken, Token};

pub struct Lexer {
    reader: Box<dyn BufRead>,
    tokens: VecDeque<PToken>,
    line_lexer: LineLexer,
    log: Vec<String>,
    prompt: bool,
}

impl Lexer {
    pub fn new(reader: Box<dyn BufRead>, prompt: bool) -> Lexer {
        Lexer {
            reader,
            prompt,
            tokens: VecDeque::new(),
            line_lexer: LineLexer::new(),
            log: Vec::new(),
        }
    }
    pub fn log(&self) -> &[String] {
        &self.log
    }
    fn read(&mut self) -> Result<bool, Error> {
        let mut line = String::new();
        if self.prompt {
            // 対話環境ではプロンプトを出す
            // ファイルから読むときは出さない
            use std::io::Write;
            print!("> ");
            std::io::stdout().flush().expect("failed to flush stdout");
        }
        let not_eof = self
            .reader
            .read_line(&mut line)
            .expect("failed to read input")
            > 0;
        if not_eof {
            let result = self.line_lexer.run(self.log.len(), &line, &mut self.tokens);
            self.log.push(line);
            result?;
        } else {
            self.line_lexer.deal_with_eof()?;
        }
        Ok(not_eof)
    }
    pub fn next(&mut self) -> Result<Option<PToken>, Error> {
        loop {
            if let Some(token) = self.tokens.pop_front() {
                return Ok(Some(token));
            } else if !self.read()? {
                return Ok(None);
            }
        }
    }
    pub fn next_if(&mut self, cond: impl FnOnce(&Token) -> bool) -> Result<Option<PToken>, Error> {
        loop {
            if let Some((_, token)) = self.tokens.front() {
                if cond(token) {
                    return Ok(self.tokens.pop_front());
                }
            } else if self.read()? {
                continue;
            }
            return Ok(None);
        }
    }
    pub fn next_if_map<B>(
        &mut self,
        f: impl FnOnce(&Token) -> Option<B>,
    ) -> Result<Option<(pos::Range, B)>, Error> {
        loop {
            if let Some((_, token)) = self.tokens.front() {
                if let Some(result) = f(token) {
                    let (pos, _) = unsafe { self.tokens.pop_front().unwrap_unchecked() };
                    return Ok(Some((pos, result)));
                }
            } else if self.read()? {
                continue;
            }
            return Ok(None);
        }
    }
}
