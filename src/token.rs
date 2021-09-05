//! cryss で使用されるトークン
//!
//! 使用時： `use crate::token`

/// トークン
///
/// `derive(Debug)` はデバッグ用，いずれ消す
#[derive(Debug)]
pub enum Token {
    /// 識別子．
    ///
    /// `Identifier` ::= [`a`-`z` `A`-`Z` `_` `$`] [`a`-`z` `A`-`Z` `_` `$` `0`-`9`]+
    Identifier(String),
    /// 整数リテラル．
    ///
    /// `Integer` ::= [`0`-`9`]+ | `0b` [`0`-`1`]+ | `0o` [`0`-`7`]+ | `0x` [`0`-`9` `a`-`f` `A`-`F`]+
    Integer(String),
    /// 浮動小数点数リテラル．
    ///
    /// `Real` ::= `Integer` `Exponent` | `Decimal` `Exponent`?
    /// where `Decimal` ::= [`0`-`9`]+ `.` [`0`-`9`]* | `.` [`0`-`9`]+
    /// and `Exponent` ::= [`e` `E`] ([`+` `-`])? [`0`-`9`]
    Real(String),
    /// 文字列リテラル
    String(String),
    /// `+`: 足し算
    Plus,
    /// `-`: （ 2 項）引き算，（単項）負号
    Hyphen,
    /// `*`: 掛け算
    Asterisk,
    /// `**`: 累乗
    DoubleAsterisk,
    /// `/`: 割り算，（単項）逆数
    Slash,
    /// `%`: 割った余り
    Percent,
    /// `=`: 代入
    Equal,
    /// `==`: 等しい
    DoubleEqual,
    /// `!`: 論理否定
    Exclamation,
    /// `!=`: 等しくない
    ExclamationEqual,
    /// `<`: より小さい
    Less,
    /// `<=`: 以下
    LessEqual,
    /// `<<`: 左シフト
    DoubleLess,
    /// `<<<`: backward シフト
    TripleLess,
    /// `>`: より大きい
    Greater,
    /// `>=`: 以上
    GreaterEqual,
    /// `>>`: 右シフト
    DoubleGreater,
    /// `>>>`: forward シフト
    TripleGreater,
    /// `&`: ビット and
    Ampersand,
    /// `&&`: 論理積
    DoubleAmpersand,
    /// `|`: ビット or
    Bar,
    /// `||`: 論理和
    DoubleBar,
    /// `^`: ビット xor
    Circumflex,
    /// `.`
    Dot,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    /// `,`
    Comma,
    /// `?`: 出力
    Question,
    /// `(`
    OpeningParenthesis,
    /// `)`
    ClosingParenthesis,
    /// `[`
    OpeningBracket,
    /// `]`
    ClosingBracket,
    /// `{`
    OpeningBrace,
    /// `}`
    ClosingBrace,
}
