#[derive(Debug)]
pub enum Token {
    Identifier(String),
    BinInt(String),
    BinIntSuffix(String, String),
    OctInt(String),
    OctIntSuffix(String, String),
    DecInt(String),
    DecIntSuffix(String, String),
    HexInt(String),
    HexIntSuffix(String, String),
    Float(String),
    FloatSuffix(String, String),
    Plus,
    Hyphen,
    Dot,
}

pub type RToken = (crate::pos::Range, Token);
