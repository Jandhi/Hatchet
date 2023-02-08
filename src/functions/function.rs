use crate::value::Value;
use crate::value::Data;
use crate::types::types::Type;

pub struct Function {
    pub func: fn(&Vec<Value>) -> Value,
    pub min_args : u8,
    pub max_args : u8,
}

impl Data for Function {
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