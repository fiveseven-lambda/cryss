use std::cell::{Cell, RefCell};
use std::rc::Rc;

type RcCell<T> = Rc<Cell<T>>;
type RcRefCell<T> = Rc<RefCell<T>>;

use crate::sound::Sound;
use crate::types::Type;

/// ループ中で書き換えられるために RcCell で包む
#[derive(Clone)]
pub enum Value {
    Real(RcCell<f64>),
    Boolean(RcCell<bool>),
    Sound(RcRefCell<Sound>),
    String(RcRefCell<String>),
}

impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Value::Real(_) => Type::Real,
            Value::Boolean(_) => Type::Boolean,
            Value::Sound(_) => Type::Sound,
            Value::String(_) => Type::String,
        }
    }
}
