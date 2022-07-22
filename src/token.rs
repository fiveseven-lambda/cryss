#[derive(Debug)]
pub enum Token {
    Identifier(String),
    BinInt(String),
    OctInt(String),
    DecInt(String),
    HexInt(String),
    Float(String),
    Plus,
    DoublePlus,
    PlusEqual,
    Hyphen,
    DoubleHyphen,
    HyphenEqual,
    Dot,
}

pub type RToken = (crate::pos::Range, Token);
