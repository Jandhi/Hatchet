use crate::expression::Expression;
use crate::value::Value;
use crate::types::types::Type;

#[derive(Copy, Clone)]
pub struct Integer
{
    pub value : i32,
}

impl Value for Integer {
    fn get_type(&self) -> super::types::Type {
        Type::Integer
    }

    fn print(&self) {
        print!("{}", self.value)
    }

    fn as_int(&self) -> &Integer {
        &self
    }
}

impl Expression for Integer {
    fn evaluate(&self) -> Box<dyn Value> {
        Box::from(*self)
    }
}