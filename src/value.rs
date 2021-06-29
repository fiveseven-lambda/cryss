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
    RealFunction(RealFunction),
    BooleanFunction(BooleanFunction),
    SoundFunction(SoundFunction),
    StringFunction(StringFunction),
    VoidFunction(VoidFunction),
}

#[derive(Clone)]
enum Argument {
    Real(RcCell<f64>),
    Boolean(RcCell<bool>),
    Sound(RcCell<sound::Sound>),
    String(RcCell<String>),
}

#[derive(Clone)]
pub struct RealFunction {
    definition: Rc<RealFunctionDefinition>,
    arguments: Vec<Argument>,
    options: HashMap<String, Argument>,
}
#[derive(Clone)]
pub struct BooleanFunction {
    definition: Rc<BooleanFunctionDefinition>,
    arguments: Vec<Argument>,
    options: HashMap<String, Argument>,
}
#[derive(Clone)]
pub struct SoundFunction {
    definition: Rc<SoundFunctionDefinition>,
    arguments: Vec<Argument>,
    options: HashMap<String, Argument>,
}
#[derive(Clone)]
pub struct StringFunction {
    definition: Rc<StringFunctionDefinition>,
    arguments: Vec<Argument>,
    options: HashMap<String, Argument>,
}
#[derive(Clone)]
pub struct VoidFunction {
    definition: Rc<VoidFunctionDefinition>,
    arguments: Vec<Argument>,
    options: HashMap<String, Argument>,
}

pub enum RealFunctionDefinition {
    Primitive1(Box<fn(f64) -> f64>, RcCell<f64>),
    UserDefined(Vec<program::RealFunctionStatement>, Vec<Value>),
}
pub enum BooleanFunctionDefinition {
    UserDefined(Vec<program::BooleanFunctionStatement>, Vec<Value>),
}
pub enum StringFunctionDefinition {
    UserDefined(Vec<program::StringFunctionStatement>, Vec<Value>),
}
pub enum SoundFunctionDefinition {
    UserDefined(Vec<program::SoundFunctionStatement>, Vec<Value>),
}
pub enum VoidFunctionDefinition {
    UserDefined(Vec<program::VoidFunctionStatement>, Vec<Value>),
}
