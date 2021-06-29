//! エラー出力のためのモジュール

use crate::pos;

pub enum Error {
    UnexpectedCharacter(pos::Pos),
    NoCharacterAfterBackSlash(pos::Pos),
    UnterminatedComment(pos::Pos),
    UnterminatedStringLiteral(pos::Pos),
    NoLineFeedAtEOF,
    IncompleteScientificNotation(pos::Range),
    SingleAmpersand(pos::Range),
    SingleDot(pos::Range),
    ParseFloatError(pos::Range, std::num::ParseFloatError),
    UnclosedBracketUntil(pos::Range, Option<pos::Range>),
    ArgumentNameNotIdentifier(Option<pos::Range>, pos::Range),
    UndefinedVariable(String, pos::Range),
    CannotPrint(pos::Range),
    EmptyOperandMinus(pos::Range),
    TypeMismatchMinus(pos::Range),
    EmptyOperandReciprocal(pos::Range),
    TypeMismatchReciprocal(pos::Range),
    NoSemicolonAtEndOfStatement(pos::Range),
    UnexpectedToken(pos::Range),
}

impl Error {
    pub fn print(&self, log: &Vec<String>) {
        print!("error: ");
        match self {
            Error::UnexpectedCharacter(pos) => {
                println!("unexpected character at {}", pos);
                pos.print(log);
            }
            Error::NoCharacterAfterBackSlash(pos) => {
                println!("no character after `\\` at {}", pos);
                pos.print(log);
            }
            Error::UnterminatedComment(pos) => {
                println!("unterminated comment (started at {})", pos);
                pos.print(log);
            }
            Error::UnterminatedStringLiteral(pos) => {
                println!("unterminated string literal (started at {})", pos);
                pos.print(log);
            }
            Error::NoLineFeedAtEOF => {
                println!("no line feed at end of file");
            }
            Error::IncompleteScientificNotation(range) => {
                println!("incomplete scientific notation at {}", range);
                range.print(log);
            }
            Error::SingleAmpersand(range) => {
                println!("single ampersand at {}", range);
                range.print(log);
            }
            Error::SingleDot(range) => {
                println!("single dot at {}", range);
                range.print(log);
            }
            Error::ParseFloatError(range, err) => {
                println!("failed to parse number at {} ({})", range, err);
                range.print(log);
            }
            Error::UnclosedBracketUntil(open, range) => {
                match range {
                    Some(range) => {
                        println!("unexpected token at {}", range);
                        range.print(log);
                    }
                    None => println!("unexpected end of file"),
                }
                println!("note: bracket opened at {}", open);
                open.print(log);
            }
            Error::ArgumentNameNotIdentifier(range, equal) => {
                match range {
                    Some(range) => {
                        println!("invalid argument name at {}", range);
                        range.print(log);
                    }
                    None => println!("empty argument name"),
                }
                println!("note: argument name is needed before `=` at {}", equal);
                equal.print(log);
            }
            Error::UndefinedVariable(name, range) => {
                println!("undefined variable {} at {}", name, range);
                range.print(log);
            }
            Error::CannotPrint(range) => {
                println!("cannot apply `?` (at {})", range);
                range.print(log);
            }
            Error::EmptyOperandMinus(range) => {
                println!("empty operand of `-` (at {})", range);
                range.print(log);
            }
            Error::TypeMismatchMinus(range) => {
                println!("type mismatch at {}. `-` expected real", range);
                range.print(log);
            }
            Error::EmptyOperandReciprocal(range) => {
                println!("empty operand of `/` (at {})", range);
                range.print(log);
            }
            Error::TypeMismatchReciprocal(range) => {
                println!("type mismatch at {}. `/` expected real", range);
                range.print(log);
            }
            Error::NoSemicolonAtEndOfStatement(range) => {
                println!("no semicolon at end of statement ({})", range);
                range.print(log);
            }
            Error::UnexpectedToken(range) => {
                println!("unexpected token at {}", range);
                range.print(log);
            }
        }
    }
}
