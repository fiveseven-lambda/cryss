use crate::pos;

pub enum Error {
    UnexpectedCharacter(pos::Start),
    InvalidNumericLiteral(pos::Range),
    UnterminatedComment(Vec<pos::Start>),
    UnterminatedStringLiteral(pos::Start),
}

impl Error {
    pub fn eprint(&self, log: &[String]) {
        eprint!("error: ");
        match self {
            Error::UnexpectedCharacter(pos) => {
                eprintln!("unexpected character at {}", pos);
                pos.eprint(log);
            }
            Error::InvalidNumericLiteral(range) => {
                eprintln!("invalid numeric literal at {}", range);
                range.eprint(log);
            }
            Error::UnterminatedComment(poss) => {
                eprintln!("unterminated comment");
                for pos in poss {
                    eprintln!("started at {}", pos);
                    pos.eprint(log);
                }
            }
            Error::UnterminatedStringLiteral(pos) => {
                eprintln!("unterminated string literal started at {}", pos);
                pos.eprint(log);
            }
        }
    }
}
