use crate::value::Value;
use crate::types::types::Type;

pub struct Function {
    pub func: fn(&Vec<Box<dyn Value>>) -> Box<dyn Value>,
    pub min_args : u8,
    pub max_args : u8,
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

impl Function {
}