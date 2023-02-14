use crate::{expression::Expression, value::Value};
use super::function::Function;
use crate::functions::args::Args;

pub struct FunctionCall {
    function : Function,
    args : Args
}   

impl Expression for FunctionCall {
    fn evaluate(&self) -> Box<dyn Value> {
        (self.function.func)(self.args.clone())
    }
}