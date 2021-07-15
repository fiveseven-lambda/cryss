use crate::program::{Argument, RealExpression};
use crate::sound::Sound;
use crate::value::Value;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
type RcCell<T> = Rc<Cell<T>>;
type RcRefCell<T> = Rc<RefCell<T>>;

pub struct Function {
    pub body: Body,
    pub arguments: Vec<Value>,
    pub named_arguments: Vec<(String, Argument)>,
}

impl Function {
    pub fn primitive_real_1(fnc: fn(f64) -> f64) -> Function {
        let x = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![Value::Real(x.clone())],
            named_arguments: Vec::new(),
            body: Body::Real(Rc::new(RealFunction::Primitive1(fnc, x))),
        }
    }
    pub fn primitive_real_2(fnc: fn(f64, f64) -> f64) -> Function {
        let x = Rc::new(Cell::new(0.));
        let y = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![Value::Real(x.clone()), Value::Real(y.clone())],
            named_arguments: Vec::new(),
            body: Body::Real(Rc::new(RealFunction::Primitive2(fnc, x, y))),
        }
    }
    pub fn sin() -> Function {
        let x = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![Value::Real(x.clone())],
            named_arguments: Vec::new(),
            body: Body::Sound(Rc::new(SoundFunction::Sin(x))),
        }
    }
    pub fn exp() -> Function {
        let x = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![Value::Real(x.clone())],
            named_arguments: Vec::new(),
            body: Body::Sound(Rc::new(SoundFunction::Exp(x))),
        }
    }
    pub fn linear() -> Function {
        let x1 = Rc::new(Cell::new(0.));
        let t1 = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![Value::Real(x1.clone())],
            named_arguments: vec![(
                "t".to_string(),
                Argument::Real(t1.clone(), RealExpression::Const(1.)),
            )],
            body: Body::Sound(Rc::new(SoundFunction::Linear(x1, t1))),
        }
    }
    pub fn write() -> Function {
        let sound = Rc::new(RefCell::new(Sound::Const(0.)));
        let time = Rc::new(Cell::new(0.));
        let filename = Rc::new(RefCell::new("".to_string()));
        Function {
            arguments: vec![
                Value::Sound(sound.clone()),
                Value::Real(time.clone()),
                Value::String(filename.clone()),
            ],
            named_arguments: Vec::new(),
            body: Body::Void(Rc::new(VoidFunction::Write(sound, time, filename))),
        }
    }
}

pub enum Body {
    Real(Rc<RealFunction>),
    Boolean(Rc<BooleanFunction>), // ユーザ定義関数ができたら使う
    Sound(Rc<SoundFunction>),
    String(Rc<StringFunction>), // ユーザ定義関数ができたら使う
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
impl BooleanFunction {
    pub fn evaluate(&self) -> bool {
        todo!();
    }
}

pub enum SoundFunction {
    Sin(RcCell<f64>),
    Linear(RcCell<f64>, RcCell<f64>),
    Exp(RcCell<f64>),
}

impl SoundFunction {
    pub fn evaluate(&self) -> Sound {
        match self {
            SoundFunction::Sin(frequency) => Sound::Sin {
                frequency: frequency.get(),
                phase: 0.,
            },
            SoundFunction::Linear(x1, t1) => Sound::Linear {
                slope: x1.get() / t1.get(),
                intercept: 0.,
            },
            SoundFunction::Exp(time) => Sound::Exp {
                slope: time.get().recip(),
                intercept: 0.,
            },
        }
    }
}

pub enum StringFunction {}
impl StringFunction {
    pub fn evaluate(&self) -> String {
        todo!();
    }
}

pub enum VoidFunction {
    Write(RcRefCell<Sound>, RcCell<f64>, RcRefCell<String>),
}
impl VoidFunction {
    pub fn evaluate(&self) {
        match self {
            VoidFunction::Write(sound, time, filename) => {
                let samplerate = 44100;
                let mut iter = sound.borrow().clone().iter(samplerate as f64);
                let spec = hound::WavSpec {
                    channels: 1,
                    sample_rate: samplerate,
                    bits_per_sample: 32,
                    sample_format: hound::SampleFormat::Int,
                };
                let mut writer = hound::WavWriter::create(&*filename.borrow(), spec).unwrap();
                let amplitude = std::i32::MAX as f64;
                for _ in 0..(time.get() * samplerate as f64) as i64 {
                    writer
                        .write_sample((amplitude * iter.next()) as i32)
                        .unwrap();
                }
                writer.finalize().unwrap();
            }
        }
    }
}
