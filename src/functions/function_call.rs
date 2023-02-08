use crate::{expression::Expression, value::Value};
use super::function::Function;

pub struct FunctionCall {
    function : Function,
    args : Vec<Value>
}   

impl Expression for FunctionCall {
    fn evaluate(&mut self) -> Value {
        (self.function.func)(&self.args)
    }
}