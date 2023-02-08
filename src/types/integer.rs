use crate::value::Data;
use crate::types::types::Type;

pub struct Integer
{
    pub value : i32,
}

impl Data for Integer {
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