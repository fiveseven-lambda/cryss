//! エラー
//!
//! 使用時： `use crate::error::Error`

use crate::pos;

pub enum Error {
    /// 字句解析中に予期せぬ文字が現れた．
    UnexpectedCharacter(pos::Pos),
    /// コメントの途中でファイル末尾に達した．
    UnterminatedComment(pos::Pos /* 最後のコメント開始位置 */),
    /// 文字列リテラルの途中でファイル末尾に達した．
    UnterminatedStringLiteral(pos::Pos /*文字列リテラルの開始位置*/),
    /// 無効なトークン：`0b` `0o` `0x`，指数表記の `e`|`E` (`+`|`-`)? の後に数が続かないもの．
    InvalidToken(pos::Range),
}

impl Error {
    pub fn eprint(&self, log: &[String]) {
        eprint!("error: ");
        match self {
            Error::UnexpectedCharacter(pos) => {
                eprintln!("unexpected character at {}", pos);
                pos.eprint(log);
            }
            Error::UnterminatedComment(pos) => {
                eprintln!("unterminated comment started at {}", pos);
                pos.eprint(log);
            }
            Error::UnterminatedStringLiteral(pos) => {
                eprintln!("unterminated string literal started at {}", pos);
                pos.eprint(log);
            }
            Error::InvalidToken(range) => {
                eprintln!("invalid token at {}", range);
                range.eprint(log);
            }
        }
    }
}
