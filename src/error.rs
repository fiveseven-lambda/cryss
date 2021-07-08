//! エラー出力のためのモジュール

use crate::pos;
use crate::types::Type;

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
    UnclosedBracketUntil(pos::Range, pos::Range),
    UnclosedBracketUntilEOF(pos::Range),
    EmptyArgumentName(pos::Range),
    InvalidArgumentName(pos::Range, pos::Range),
    UndefinedVariable(String, pos::Range),
    UndefinedFunction(String, pos::Range),
    EmptyOperandUnary(pos::Range),
    EmptyOperandRight(pos::Range),
    EmptyArgument(pos::Range),
    EmptyNamedArgument(pos::Range),
    EmptyParentheses(pos::Range, pos::Range),
    EmptyRHS(pos::Range),
    TypeMismatchUnary(pos::Range, Type),
    TypeMismatchBinary(pos::Range, Type, pos::Range, Type),
    TypeMismatchCond(pos::Range, Type),
    WrongNumberOfArguments(pos::Range, usize, usize),
    TypeMismatchArgument(pos::Range, Type),
    LHSNotIdentifier(pos::Range, pos::Range),
    NoSemicolonAtEndOfStatement(pos::Range),
    UnexpectedToken(pos::Range),
    NoSubstitutionAfterLet(pos::Range),
    UnexpectedTokenAfterKeyword(pos::Range, pos::Range),
    UnexpectedEOFAfterKeyword(pos::Range),
    UnexpectedEOFAfterCondition(pos::Range, pos::Range),
    VoidRHS(pos::Range),
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
                println!("unexpected token at {}", range);
                range.print(log);
                println!("note: bracket opened at {}", open);
                open.print(log);
            }
            Error::UnclosedBracketUntilEOF(open) => {
                println!("unexpected end of file");
                println!("note: bracket opened at {}", open);
                open.print(log);
            }
            Error::EmptyArgumentName(equal) => {
                println!("empty argument name before `=` at {}", equal);
                equal.print(log);
            }
            Error::InvalidArgumentName(range, equal) => {
                println!("invalid argument name at {}", range);
                range.print(log);
                println!("before `=` at {}", equal);
                equal.print(log);
            }
            Error::UndefinedVariable(name, range) => {
                println!("undefined variable {} at {}", name, range);
                range.print(log);
            }
            Error::UndefinedFunction(name, range) => {
                println!("undefined function {} at {}", name, range);
                range.print(log);
            }
            Error::EmptyOperandUnary(range) => {
                println!("empty operand of unary operator at {}", range);
                range.print(log);
            }
            Error::EmptyOperandRight(range) => {
                println!("empty operand after binary operator at {}", range);
                range.print(log);
            }
            Error::EmptyArgument(range) => {
                println!("empty argument before comma at {}", range);
                range.print(log);
            }
            Error::EmptyNamedArgument(range) => {
                println!("empty argument after equal at {}", range);
                range.print(log);
            }
            Error::EmptyParentheses(open, close) => {
                println!("empty expression between opening parenthesis at {}", open);
                open.print(log);
                println!("and closing parenthesis at {}", close);
                close.print(log);
            }
            Error::TypeMismatchUnary(range, ty) => {
                println!("type mismatch at {} (found {})", range, ty);
                range.print(log);
            }
            Error::TypeMismatchBinary(left, left_ty, right, right_ty) => {
                println!("type mismatch at {} (found {})", left, left_ty);
                left.print(log);
                println!("and {} (found {})", right, right_ty);
                right.print(log);
            }
            Error::TypeMismatchCond(cond, ty) => {
                println!("type mismatch at {} (found {})", cond, ty);
                cond.print(log);
            }
            Error::WrongNumberOfArguments(range, expected, found) => {
                println!(
                    "wrong number of arguments at {} (expected {}, found {})",
                    range, expected, found
                );
                range.print(log);
            }
            Error::TypeMismatchArgument(arg, ty) => {
                println!("type mismatch at {} (found {})", arg, ty);
                arg.print(log);
            }
            Error::NoSemicolonAtEndOfStatement(range) => {
                println!("no semicolon at end of statement ({})", range);
                range.print(log);
            }
            Error::UnexpectedToken(range) => {
                println!("unexpected token at {}", range);
                range.print(log);
            }
            Error::LHSNotIdentifier(range, equal) => {
                println!("identifier required at {}", range);
                range.print(log);
                println!("before `=` at {}", equal);
                equal.print(log);
            }
            Error::EmptyRHS(equal) => {
                println!("empty expression after `=` at {}", equal);
                equal.print(log);
            }
            Error::NoSubstitutionAfterLet(r#let) => {
                println!("no substitution after `let` at {}", r#let);
                r#let.print(log);
            }
            Error::UnexpectedTokenAfterKeyword(keyword, token) => {
                println!("unexpected token at {}", token);
                token.print(log);
                println!("after keyword at {}", keyword);
                keyword.print(log);
            }
            Error::UnexpectedEOFAfterKeyword(keyword) => {
                println!("unexpected end of file after keyword at {}", keyword);
                keyword.print(log);
            }
            Error::UnexpectedEOFAfterCondition(keyword, condition) => {
                println!("unexpected end of file after keyword at {}", keyword);
                keyword.print(log);
                println!("and condition at {}", condition);
                condition.print(log);
            }
            Error::VoidRHS(range) => {
                println!("void expression at rhs {}", range);
                range.print(log);
            }
        }
    }
}
