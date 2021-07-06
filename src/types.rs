//! 型リスト（デバッグ用）

pub enum Type {
    Real,
    Boolean,
    Sound,
    String,
    Void,
}

use std::fmt::{Display, Formatter, Result as FResult};
impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> FResult {
        match self {
            Type::Real => write!(f, "real"),
            Type::Boolean => write!(f, "boolean"),
            Type::Sound => write!(f, "Sound"),
            Type::String => write!(f, "string"),
            Type::Void => write!(f, "void"),
        }
    }
}
