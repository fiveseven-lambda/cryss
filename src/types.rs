//! 型リスト（デバッグ用）

pub enum Type {
    Real,
    Boolean,
    Sound,
    String,
    Void,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Real => write!(f, "real"),
            Type::Boolean => write!(f, "boolean"),
            Type::Sound => write!(f, "Sound"),
            Type::String => write!(f, "string"),
            Type::Void => write!(f, "void"),
        }
    }
}
