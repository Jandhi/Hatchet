use crate::function::Function;

#[derive(Clone)]
pub enum Value {
    String(StringVal),
    Integer(IntegerVal),
    Function(Function),
    None,
}

pub type IntegerVal = i32;
pub type StringVal = String;
