use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

type RcCell<T> = Rc<Cell<T>>;

use crate::{program, sound};

/// ループ中で書き換えられるために RcCell で包む
#[derive(Clone)]
pub enum Value {
    Real(RcCell<f64>),
    Boolean(RcCell<bool>),
    Sound(RcCell<sound::Sound>),
    String(RcCell<String>),
}
