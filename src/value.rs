use crate::{types::types::Type, functions::function::Function, types::string::StringValue};

pub trait Value {
    fn get_type(&self) -> Type;
    fn print(&self);

    fn as_func(&self) -> &Function {
        panic!("This isn't a function!")
    }
    fn as_string(&self) -> &StringValue {
        panic!("This isn't a string!")
    }
}