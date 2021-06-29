//! cryss で使用されるトークン

/// トークン
#[derive(Debug)]
pub enum Token {
    /// 識別子．
    Identifier(String),
    /// `$` で始まる
    Parameter(String),
    Number(f64),
    String(String),
    /// キーワード `real`
    KeywordReal,
    /// キーワード `boolean`
    KeywordBoolean,
    /// キーワード `Sound`
    KeywordSound,
    /// キーワード `string`
    KeywordString,
    KeywordFnc,
    KeywordLet,
    KeywordBreak,
    KeywordContinue,
    /// キーワード `if`
    KeywordIf,
    /// キーワード `while`
    KeywordWhile,
    /// キーワード `for`
    /// `for` 文の syntax は考え中
    KeywordFor,
    /// `+`: 足し算
    Plus,
    /// `-`: （ 2 項）引き算，（単項）負号
    Minus,
    /// `*`: 掛け算
    Asterisk,
    /// `/`: 割り算，（単項）逆数
    Slash,
    /// `%`: 割った余り
    Percent,
    /// `^`: 累乗
    Circumflex,
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
    /// `<<`: 左シフト（ Sound ）
    DoubleLess,
    /// `>`: より大きい
    Greater,
    /// `>>`: 右シフト（ Sound ）
    DoubleGreater,
    /// `&&`: 論理積
    DoubleAmpersand,
    /// `|`
    Bar,
    /// `||`: 論理和
    DoubleBar,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    /// `,`
    Comma,
    /// `?`: 出力
    Question,
    /// `(`
    OpeningParen,
    /// `)`
    ClosingParen,
    /// `[`
    OpeningBracket,
    /// `]`
    ClosingBracket,
    /// `{`
    OpeningBrace,
    /// `}`
    ClosingBrace,
}
