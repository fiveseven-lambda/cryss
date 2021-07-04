use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

type RcCell<T> = Rc<Cell<T>>;

use crate::{program, sound, types};

/// ループ中で書き換えられるために RcCell で包む
#[derive(Clone)]
pub enum Value {
    Real(RcCell<f64>),
    Boolean(RcCell<bool>),
    Sound(RcCell<sound::Sound>),
    String(RcCell<String>),
}

impl Value {
    pub fn ty(&self) -> types::Type {
        match self {
            Value::Real(_) => types::Type::Real,
            Value::Boolean(_) => types::Type::Boolean,
            Value::Sound(_) => types::Type::Sound,
            Value::String(_) => types::Type::String,
        }
    }
}
