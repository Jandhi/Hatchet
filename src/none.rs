use crate::value::Data;
use crate::types::types::Type;

pub struct None {}

impl Data for None {
    fn get_type(&self) -> Type {
        Type::None
    }

    fn print(&self) {
        println!("None")
    }
}