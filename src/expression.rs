use crate::value::Value;

pub trait Expression {
    fn evaluate(&self) -> Box<dyn Value>;
}