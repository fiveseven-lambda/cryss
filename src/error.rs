use crate::pos;

pub enum Error {
    UnexpectedCharacter(pos::Start),
    InvalidNumericLiteral(pos::Range),
    UnterminatedComment(Vec<pos::Start>),
    UnterminatedStringLiteral(pos::Start),
    UnexpectedEOFAfterPrefixOperator(pos::Range),
    UnexpectedTokenAfterPrefixOperator(pos::Range, pos::Range),
    UnexpectedEOFAfterBinaryOperator(pos::Range),
    UnexpectedTokenAfterBinaryOperator(pos::Range, pos::Range),
    NoClosingParenthesis(pos::Range),
    UnexpectedTokenInParenthesis(pos::Range, pos::Range),
    EmptyParenthesis(pos::Range, pos::Range),
    NoExpressionBeforeComma(pos::Range),
}

impl Error {
    pub fn eprint(&self, log: &[String]) {
        eprint!("error: ");
        match self {
            Error::UnexpectedCharacter(pos) => {
                eprintln!("unexpected character at {pos}");
                pos.eprint(log);
            }
            Error::InvalidNumericLiteral(pos) => {
                eprintln!("invalid numeric literal at {pos}");
                pos.eprint(log);
            }
            Error::UnterminatedComment(poss) => {
                eprintln!("unterminated comment");
                for pos in poss {
                    eprintln!("started at {pos}");
                    pos.eprint(log);
                }
            }
            Error::UnterminatedStringLiteral(pos) => {
                eprintln!("unterminated string literal started at {pos}");
                pos.eprint(log);
            }
            Error::UnexpectedEOFAfterPrefixOperator(pos) => {
                eprintln!("unexpected EOF after prefix operator at {pos}");
            }
            Error::UnexpectedTokenAfterPrefixOperator(prefix, token) => {
                eprintln!("unexpected token at {token}");
                token.eprint(log);
                eprintln!("after prefix operator at {prefix}");
                prefix.eprint(log);
            }
            Error::NoClosingParenthesis(open) => {
                eprintln!("no closing parenthesis (opened at {open})");
                open.eprint(log);
            }
            Error::UnexpectedTokenInParenthesis(open, token) => {
                eprintln!("unexpected token at {token}");
                token.eprint(log);
                eprintln!("parenthesis opened at {open}");
                open.eprint(log);
            }
            Error::EmptyParenthesis(open, close) => {
                eprintln!("empty parenthesis (opened at {open})");
                open.eprint(log);
                eprintln!("(closed at {close})");
                close.eprint(log);
            }
            Error::UnexpectedEOFAfterBinaryOperator(op) => {
                eprintln!("unexpected EOF after binary operator at {op}");
            }
            Error::UnexpectedTokenAfterBinaryOperator(op, token) => {
                eprintln!("unexpected token at {token}");
                token.eprint(log);
                eprintln!("after binary operator at {op}");
                op.eprint(log);
            }
            Error::NoExpressionBeforeComma(comma) => {
                eprintln!("no expression before comma at {comma}");
                comma.eprint(log);
            }
        }
    }
}
