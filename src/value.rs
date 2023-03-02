use std::fmt::Display;

use crate::function::Function;

#[derive(Clone)]
pub enum Value {
    String(StringVal),
    Integer(IntegerVal),
    Function(Function),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(val) => write!(f, "{}", val),
            Value::Integer(val) => write!(f, "{}", val),
            Value::Function(func) => write!(f, "<some-function>"),
            Value::None => write!(f, "None"),
        }
    }
}

pub type IntegerVal = i32;
pub type StringVal = String;
