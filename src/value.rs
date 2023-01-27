use crate::{
    types::{
        types::Type,
        string::StringValue,
        integer::Integer,
    },
    functions::function::Function
};

pub trait Value {
    fn get_type(&self) -> Type;
    fn print(&self);

    fn as_func(&self) -> &Function {
        panic!("This isn't a function!")
    }
    fn as_string(&self) -> &StringValue {
        panic!("This isn't a string!")
    }
    fn as_int(&self) -> &Integer {
        panic!("This isn't an integer!")
    }
}