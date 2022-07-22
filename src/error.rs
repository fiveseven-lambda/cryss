use crate::pos;

pub enum Error {
    UnexpectedCharacter(pos::Start),
    InvalidNumericLiteral(pos::Range),
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
        }
    }
}
