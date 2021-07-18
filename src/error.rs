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
    EmptyExpressionReturn(pos::Range),
    TypeMismatchUnary(pos::Range, Type),
    TypeMismatchBinary(pos::Range, Type, pos::Range, Type),
    TypeMismatchCond(pos::Range, Type),
    TypeMismatchReturn(pos::Range, Type),
    WrongNumberOfArguments(pos::Range, usize, usize),
    UnusedNamedArguments(pos::Range, Vec<String>),
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
    pub fn print<Write: std::io::Write>(
        &self,
        w: &mut Write,
        log: &Vec<String>,
    ) -> Result<(), std::io::Error> {
        write!(w, "error: ")?;
        match self {
            Error::UnexpectedCharacter(pos) => {
                writeln!(w, "unexpected character at {}", pos)?;
                pos.print(w, log)
            }
            Error::NoCharacterAfterBackSlash(pos) => {
                writeln!(w, "no character after `\\` at {}", pos)?;
                pos.print(w, log)
            }
            Error::UnterminatedComment(pos) => {
                writeln!(w, "unterminated comment (started at {})", pos)?;
                pos.print(w, log)
            }
            Error::UnterminatedStringLiteral(pos) => {
                writeln!(w, "unterminated string literal (started at {})", pos)?;
                pos.print(w, log)
            }
            Error::NoLineFeedAtEOF => {
                writeln!(w, "no line feed at end of file")
            }
            Error::IncompleteScientificNotation(range) => {
                writeln!(w, "incomplete scientific notation at {}", range)?;
                range.print(w, log)
            }
            Error::SingleAmpersand(range) => {
                writeln!(w, "single ampersand at {}", range)?;
                range.print(w, log)
            }
            Error::SingleDot(range) => {
                writeln!(w, "single dot at {}", range)?;
                range.print(w, log)
            }
            Error::ParseFloatError(range, err) => {
                writeln!(w, "failed to parse number at {} ({})", range, err)?;
                range.print(w, log)
            }
            Error::UnclosedBracketUntil(open, range) => {
                writeln!(w, "unexpected token at {}", range)?;
                range.print(w, log)?;
                writeln!(w, "note: bracket opened at {}", open)?;
                open.print(w, log)
            }
            Error::UnclosedBracketUntilEOF(open) => {
                writeln!(w, "unexpected end of file")?;
                writeln!(w, "note: bracket opened at {}", open)?;
                open.print(w, log)
            }
            Error::EmptyArgumentName(equal) => {
                writeln!(w, "empty argument name before `=` at {}", equal)?;
                equal.print(w, log)
            }
            Error::InvalidArgumentName(range, equal) => {
                writeln!(w, "invalid argument name at {}", range)?;
                range.print(w, log)?;
                writeln!(w, "before `=` at {}", equal)?;
                equal.print(w, log)
            }
            Error::UndefinedVariable(name, range) => {
                writeln!(w, "undefined variable {} at {}", name, range)?;
                range.print(w, log)
            }
            Error::UndefinedFunction(name, range) => {
                writeln!(w, "undefined function {} at {}", name, range)?;
                range.print(w, log)
            }
            Error::EmptyOperandUnary(range) => {
                writeln!(w, "empty operand of unary operator at {}", range)?;
                range.print(w, log)
            }
            Error::EmptyOperandRight(range) => {
                writeln!(w, "empty operand after binary operator at {}", range)?;
                range.print(w, log)
            }
            Error::EmptyArgument(range) => {
                writeln!(w, "empty argument before comma at {}", range)?;
                range.print(w, log)
            }
            Error::EmptyNamedArgument(range) => {
                writeln!(w, "empty argument after equal at {}", range)?;
                range.print(w, log)
            }
            Error::EmptyParentheses(open, close) => {
                writeln!(
                    w,
                    "empty expression between opening parenthesis at {}",
                    open
                )?;
                open.print(w, log)?;
                writeln!(w, "and closing parenthesis at {}", close)?;
                close.print(w, log)
            }
            Error::EmptyExpressionReturn(range) => {
                writeln!(w, "empty expression after `return` at {}", range)?;
                range.print(w, log)
            }
            Error::TypeMismatchUnary(range, ty) => {
                writeln!(w, "type mismatch at {} (found {})", range, ty)?;
                range.print(w, log)
            }
            Error::TypeMismatchBinary(left, left_ty, right, right_ty) => {
                writeln!(w, "type mismatch at {} (found {})", left, left_ty)?;
                left.print(w, log)?;
                writeln!(w, "and {} (found {})", right, right_ty)?;
                right.print(w, log)
            }
            Error::TypeMismatchCond(cond, ty) => {
                writeln!(w, "type mismatch at {} (found {})", cond, ty)?;
                cond.print(w, log)
            }
            Error::TypeMismatchReturn(range, ty) => {
                writeln!(w, "type mismatch after at {} (found {})", range, ty)?;
                range.print(w, log)
            }
            Error::WrongNumberOfArguments(range, expected, found) => {
                writeln!(
                    w,
                    "wrong number of arguments at {} (expected {}, found {})",
                    range, expected, found
                )?;
                range.print(w, log)
            }
            Error::UnusedNamedArguments(range, names) => {
                writeln!(
                    w,
                    "unused named arguments ({}) at {}",
                    names.join(", "),
                    range
                )?;
                range.print(w, log)
            }
            Error::TypeMismatchArgument(arg, ty) => {
                writeln!(w, "type mismatch at {} (found {})", arg, ty)?;
                arg.print(w, log)
            }
            Error::NoSemicolonAtEndOfStatement(range) => {
                writeln!(w, "no semicolon at end of statement ({})", range)?;
                range.print(w, log)
            }
            Error::UnexpectedToken(range) => {
                writeln!(w, "unexpected token at {}", range)?;
                range.print(w, log)
            }
            Error::LHSNotIdentifier(range, equal) => {
                writeln!(w, "identifier required at {}", range)?;
                range.print(w, log)?;
                writeln!(w, "before `=` at {}", equal)?;
                equal.print(w, log)
            }
            Error::EmptyRHS(equal) => {
                writeln!(w, "empty expression after `=` at {}", equal)?;
                equal.print(w, log)
            }
            Error::NoSubstitutionAfterLet(r#let) => {
                writeln!(w, "no substitution after `let` at {}", r#let)?;
                r#let.print(w, log)
            }
            Error::UnexpectedTokenAfterKeyword(keyword, token) => {
                writeln!(w, "unexpected token at {}", token)?;
                token.print(w, log)?;
                writeln!(w, "after keyword at {}", keyword)?;
                keyword.print(w, log)
            }
            Error::UnexpectedEOFAfterKeyword(keyword) => {
                writeln!(w, "unexpected end of file after keyword at {}", keyword)?;
                keyword.print(w, log)
            }
            Error::UnexpectedEOFAfterCondition(keyword, condition) => {
                writeln!(w, "unexpected end of file after keyword at {}", keyword)?;
                keyword.print(w, log)?;
                writeln!(w, "and condition at {}", condition)?;
                condition.print(w, log)
            }
            Error::VoidRHS(range) => {
                writeln!(w, "void expression at rhs {}", range)?;
                range.print(w, log)
            }
        }
    }
}
