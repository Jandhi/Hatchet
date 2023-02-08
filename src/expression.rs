use crate::value::Value;

pub trait Expression {
    fn evaluate(&mut self) -> Value;
}