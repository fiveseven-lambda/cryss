use std::collections::HashMap;

use crate::value;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
type RcCell<T> = Rc<Cell<T>>;
type RcRefCell<T> = Rc<RefCell<T>>;

pub struct Function {
    pub body: Body,
    pub arguments: Vec<value::Value>,
    pub named_arguments: HashMap<String, value::Value>,
}

impl Function {
    pub fn primitive_real_1(fnc: fn(f64) -> f64) -> Function {
        let x = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![value::Value::Real(x.clone())],
            body: Body::Real(Rc::new(RealFunction::Primitive1(fnc, x))),
            named_arguments: HashMap::new(),
        }
    }
    pub fn primitive_real_2(fnc: fn(f64, f64) -> f64) -> Function {
        let x = Rc::new(Cell::new(0.));
        let y = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![value::Value::Real(x.clone())],
            body: Body::Real(Rc::new(RealFunction::Primitive2(fnc, x, y))),
            named_arguments: HashMap::new(),
        }
    }
}

pub enum Body {
    Real(Rc<RealFunction>),
    Boolean(Rc<BooleanFunction>),
    Sound(Rc<SoundFunction>),
    String(Rc<StringFunction>),
    Void(Rc<VoidFunction>),
}

pub enum RealFunction {
    Primitive1(fn(f64) -> f64, RcCell<f64>),
    Primitive2(fn(f64, f64) -> f64, RcCell<f64>, RcCell<f64>),
}

impl RealFunction {
    pub fn evaluate(&self) -> f64 {
        match self {
            RealFunction::Primitive1(fnc, x) => fnc(x.get()),
            RealFunction::Primitive2(fnc, x, y) => fnc(x.get(), y.get()),
        }
    }
}

pub enum BooleanFunction {}

pub enum SoundFunction {}

pub enum StringFunction {}

pub enum VoidFunction {}
