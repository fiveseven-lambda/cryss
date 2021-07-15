//! Sound

use crate::function::RealFunction;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
type RcCell<T> = Rc<Cell<T>>;
type RcRefCell<T> = Rc<RefCell<T>>;

use std::f64::consts::TAU;

use num::complex::Complex64;
use rand::prelude::*;

#[derive(Clone)]
pub enum Argument {
    Real(RcCell<f64>, f64),
    Boolean(RcCell<bool>, bool),
    String(RcRefCell<String>, String),
    Sound(RcRefCell<Sound>, Sound),
}
impl Argument {
    fn set(&self) {
        match self {
            Argument::Real(rc, value) => rc.set(*value),
            Argument::Boolean(rc, value) => rc.set(*value),
            Argument::String(rc, value) => *rc.borrow_mut() = value.clone(),
            Argument::Sound(rc, value) => *rc.borrow_mut() = value.clone(),
        }
    }
}

#[derive(Clone)]
pub enum Sound {
    Const(f64),
    Linear { slope: f64, intercept: f64 }, // x = at + b
    Sin { frequency: f64, phase: f64 },    // x = sin(τft + θ)
    Exp { slope: f64, intercept: f64 },    // x = e^(at + b)
    Begin(f64),
    End(f64),
    Rand,
    Minus(Box<Sound>),
    Reciprocal(Box<Sound>),
    Add(Box<Sound>, Box<Sound>),
    Sub(Box<Sound>, Box<Sound>),
    Mul(Box<Sound>, Box<Sound>),
    Div(Box<Sound>, Box<Sound>),
    Pow(Box<Sound>, Box<Sound>),
    Rem(Box<Sound>, Box<Sound>),
    Apply(Rc<RealFunction>, Vec<Argument>, Vec<(RcCell<f64>, Sound)>),
}

impl Sound {
    pub fn shift(self, t: f64) -> Self {
        match self {
            Sound::Const(value) => Sound::Const(value),
            Sound::Linear { slope, intercept } => Sound::Linear {
                slope,
                intercept: slope * t + intercept,
            },
            Sound::Sin { frequency, phase } => Sound::Sin {
                frequency,
                phase: TAU * frequency * t + phase,
            },
            Sound::Exp { slope, intercept } => Sound::Exp {
                slope,
                intercept: slope * t + intercept,
            },
            Sound::Begin(time) => Sound::Begin(time + t),
            Sound::End(time) => Sound::End(time + t),
            Sound::Rand => Sound::Rand,
            Sound::Minus(sound) => Sound::Minus(sound.shift(t).into()),
            Sound::Reciprocal(sound) => Sound::Reciprocal(sound.shift(t).into()),
            Sound::Add(left, right) => Sound::Add(left.shift(t).into(), right.shift(t).into()),
            Sound::Sub(left, right) => Sound::Sub(left.shift(t).into(), right.shift(t).into()),
            Sound::Mul(left, right) => Sound::Mul(left.shift(t).into(), right.shift(t).into()),
            Sound::Div(left, right) => Sound::Div(left.shift(t).into(), right.shift(t).into()),
            Sound::Pow(left, right) => Sound::Pow(left.shift(t).into(), right.shift(t).into()),
            Sound::Rem(left, right) => Sound::Rem(left.shift(t).into(), right.shift(t).into()),
            Sound::Apply(function, arguments, sounds) => Sound::Apply(
                function,
                arguments,
                sounds
                    .into_iter()
                    .map(|(rc, sound)| (rc, sound.shift(t)))
                    .collect(),
            ),
        }
    }
    /// 本当は `&self` じゃなくて `self` にしたい．
    /// だめな理由を考えて，なければ `self` に
    pub fn iter(self, samplerate: f64) -> SoundIter {
        match self {
            Sound::Const(value) => SoundIter::Const(value),
            Sound::Linear { slope, intercept } => SoundIter::Linear {
                first: intercept,
                difference: slope / samplerate,
                counter: 0,
            },
            Sound::Sin { frequency, phase } => SoundIter::Sin {
                next: Complex64::from_polar(1., phase),
                ratio: Complex64::from_polar(1., TAU * frequency / samplerate),
            },
            Sound::Exp { slope, intercept } => SoundIter::Exp {
                first: intercept,
                difference: slope / samplerate,
                counter: 0,
            },
            Sound::Begin(time) => SoundIter::Begin((time * samplerate) as i64),
            Sound::End(time) => SoundIter::End((time * samplerate) as i64),
            Sound::Rand => SoundIter::Rand(rand::thread_rng()),
            Sound::Minus(sound) => SoundIter::Minus(sound.iter(samplerate).into()),
            Sound::Reciprocal(sound) => SoundIter::Reciprocal(sound.iter(samplerate).into()),
            Sound::Add(left, right) => {
                SoundIter::Add(left.iter(samplerate).into(), right.iter(samplerate).into())
            }
            Sound::Sub(left, right) => {
                SoundIter::Sub(left.iter(samplerate).into(), right.iter(samplerate).into())
            }
            Sound::Mul(left, right) => {
                SoundIter::Mul(left.iter(samplerate).into(), right.iter(samplerate).into())
            }
            Sound::Div(left, right) => {
                SoundIter::Div(left.iter(samplerate).into(), right.iter(samplerate).into())
            }
            Sound::Pow(left, right) => {
                SoundIter::Pow(left.iter(samplerate).into(), right.iter(samplerate).into())
            }
            Sound::Rem(left, right) => {
                SoundIter::Rem(left.iter(samplerate).into(), right.iter(samplerate).into())
            }
            Sound::Apply(function, arguments, sounds) => SoundIter::Apply(
                function.clone(),
                arguments.clone(),
                sounds
                    .into_iter()
                    .map(|(rc, sound)| (rc.clone(), sound.iter(samplerate)))
                    .collect(),
            ),
        }
    }
}

pub enum SoundIter {
    Const(f64),
    Linear {
        first: f64,
        difference: f64,
        counter: i64,
    },
    Exp {
        first: f64,
        difference: f64,
        counter: i64,
    },
    Sin {
        next: Complex64,
        ratio: Complex64,
    },
    Begin(i64),
    End(i64),
    Rand(ThreadRng),
    Minus(Box<SoundIter>),
    Reciprocal(Box<SoundIter>),
    Add(Box<SoundIter>, Box<SoundIter>),
    Sub(Box<SoundIter>, Box<SoundIter>),
    Mul(Box<SoundIter>, Box<SoundIter>),
    Div(Box<SoundIter>, Box<SoundIter>),
    Pow(Box<SoundIter>, Box<SoundIter>),
    Rem(Box<SoundIter>, Box<SoundIter>),
    Apply(
        Rc<RealFunction>,
        Vec<Argument>,
        Vec<(RcCell<f64>, SoundIter)>,
    ),
}

impl SoundIter {
    pub fn next(&mut self) -> f64 {
        match self {
            SoundIter::Const(value) => *value,
            SoundIter::Linear {
                first,
                difference,
                counter,
            } => {
                let ret = *first + *difference * *counter as f64;
                *counter += 1;
                ret
            }
            SoundIter::Sin { next, ratio } => {
                let ret = next.im;
                *next *= *ratio;
                ret
            }
            SoundIter::Exp {
                first,
                difference,
                counter,
            } => {
                let ret = *first + *difference * *counter as f64;
                *counter += 1;
                ret.exp()
            }
            SoundIter::Begin(i) => {
                if *i < 0 {
                    *i += 1;
                    0.
                } else {
                    1.
                }
            }
            SoundIter::End(i) => {
                if *i < 0 {
                    *i += 1;
                    1.
                } else {
                    0.
                }
            }
            SoundIter::Rand(rng) => rng.gen(),
            SoundIter::Minus(iter) => -iter.next(),
            SoundIter::Reciprocal(iter) => iter.next().recip(),
            SoundIter::Add(left, right) => left.next() + right.next(),
            SoundIter::Sub(left, right) => left.next() - right.next(),
            SoundIter::Mul(left, right) => left.next() * right.next(),
            SoundIter::Div(left, right) => left.next() / right.next(),
            SoundIter::Rem(left, right) => left.next() % right.next(),
            SoundIter::Pow(left, right) => left.next().powf(right.next()),
            SoundIter::Apply(fnc, arguments, sounds) => {
                arguments.iter().for_each(Argument::set);
                for (rc, sound) in sounds {
                    rc.set(sound.next());
                }
                fnc.evaluate()
            }
        }
        .clamp(f64::MIN, f64::MAX)
    }
}
