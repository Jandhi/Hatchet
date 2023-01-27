use crate::value::Value;
use crate::types::types::Type;

pub struct None {}

impl Value for None {
    fn get_type(&self) -> Type {
        Type::None
    }

    fn print(&self) {
        println!("None")
    }
}