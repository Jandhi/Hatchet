use std::fmt::Display;

use crate::function::Function;

#[derive(Clone)]
pub enum Value {
    String(StringVal),
    Integer(IntegerVal),
    Boolean(BoolVal),
    Function(Function),
    Reference(ReferenceVal),
    // TODO errors?
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(val) => write!(f, "{}", val),
            Value::Integer(val) => write!(f, "{}", val),
            Value::Boolean(val) => write!(f, "{}", match val {
                true => "true",
                false => "false",
            }),
            Value::Function(func) => write!(f, "<some-function>"),
            Value::None => write!(f, "None"),
            Value::Reference(name) => write!(f, "ref({})", name),
        }
    }
}

pub type IntegerVal = i32;
pub type StringVal = String;
pub type ReferenceVal = String;
pub type BoolVal = bool;