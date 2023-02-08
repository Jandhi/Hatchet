use crate::{expression::Expression, value::Value};
use super::function::Function;

pub struct FunctionCall {
    function : Function,
    args : Vec<Box<dyn Value>>
}   

impl Expression for FunctionCall {
    fn evaluate(&self) -> Box<dyn Value> {
        (self.function.func)(&self.args)
    }
}