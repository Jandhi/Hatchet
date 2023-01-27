use crate::value::Value;
use crate::types::types::Type;

pub struct Function {
    pub call: fn(Vec<Box<dyn Value>>) -> Box<dyn Value>,
}

impl Value for Function {
    fn get_type(&self) -> Type {
        Type::Function
    }

    fn print(&self) {
        
    }

    fn as_func(&self) -> &Function {
        &self
    }
}