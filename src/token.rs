pub enum Token {
    Identifier(String),
    BinInt(String),
    OctInt(String),
    DecInt(String),
    HexInt(String),
    Float(String),
    String(String),
    KeywordFor,
    KeywordIf,
    KeywordElse,
    Plus,
    DoublePlus,
    PlusEqual,
    Hyphen,
    DoubleHyphen,
    HyphenEqual,
    Asterisk,
    AsteriskEqual,
    Slash,
    SlashEqual,
    Percent,
    PercentEqual,
    Equal,
    DoubleEqual,
    Exclamation,
    ExclamationEqual,
    Less,
    LessEqual,
    DoubleLess,
    DoubleLessEqual,
    TripleLess,
    TripleLessEqual,
    Greater,
    GreaterEqual,
    DoubleGreater,
    DoubleGreaterEqual,
    TripleGreater,
    TripleGreaterEqual,
    Ampersand,
    AmpersandEqual,
    DoubleAmpersand,
    Bar,
    BarEqual,
    DoubleBar,
    Circumflex,
    CircumflexEqual,
    Dot,
    Colon,
    Semicolon,
    Comma,
    Question,
    Hash,
    Tilde,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    OpeningBrace,
    ClosingBrace,
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(..))
    }
    pub fn is_bin_int(&self) -> bool {
        matches!(self, Self::BinInt(..))
    }
    pub fn is_oct_int(&self) -> bool {
        matches!(self, Self::OctInt(..))
    }
    pub fn is_dec_int(&self) -> bool {
        matches!(self, Self::DecInt(..))
    }
    pub fn is_hex_int(&self) -> bool {
        matches!(self, Self::HexInt(..))
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(..))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }
    pub fn is_opening_parenthesis(&self) -> bool {
        matches!(self, Self::OpeningParenthesis)
    }
    pub fn is_comma(&self) -> bool {
        matches!(self, Self::Comma)
    }
}

pub type PToken = (crate::pos::Range, Token);
