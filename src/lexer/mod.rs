mod line_lexer;
use line_lexer::LineLexer;

use std::collections::VecDeque;
use std::io::BufRead;

use crate::error::Error;
use crate::token;

pub struct Lexer {
    reader: Box<dyn BufRead>,
    tokens: VecDeque<token::RToken>,
    line_lexer: LineLexer,
    log: Vec<String>,
}

impl Lexer {
    pub fn new(reader: Box<dyn BufRead>) -> Lexer {
        Lexer {
            reader,
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
        if self
            .reader
            .read_line(&mut line)
            .expect("failed to read input")
            > 0
        {
            let result = self.line_lexer.run(self.log.len(), &line, &mut self.tokens);
            self.log.push(line);
            result.map(|()| true)
        } else {
            self.line_lexer.deal_with_eof().map(|()| false)
        }
    }
    pub fn next(&mut self) -> Result<Option<token::RToken>, Error> {
        loop {
            match self.tokens.pop_front() {
                Some(token) => return Ok(Some(token)),
                None => {
                    if !self.read()? {
                        return Ok(None);
                    }
                }
            }
        }
    }
}
