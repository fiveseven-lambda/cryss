use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

type RcCell<T> = Rc<Cell<T>>;

use crate::{program, sound};

/// ループ中で書き換えられるために RcCell で包む
pub enum Value {
    Real(RcCell<f64>),
    Boolean(RcCell<bool>),
    Sound(RcCell<sound::Sound>),
    String(RcCell<String>),
    RealFunction(Rc<RealFunction>, Vec<Argument>, HashMap<String, Argument>),
}

enum Argument {
    Real(RcCell<f64>),
    Boolean(RcCell<bool>),
    Sound(RcCell<sound::Sound>),
    String(RcCell<String>),
}

pub enum RealFunction {
    Primitive1(Box<fn(f64) -> f64>, RcCell<f64>),
    UserDefined(Vec<program::RealFunctionStatement>, Vec<Value>),
}
pub enum BooleanFunction {}
pub enum SoundFunction {}
pub enum StringFunction {}
pub enum VoidFunction {}
