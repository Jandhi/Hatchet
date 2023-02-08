use crate::value::Data;
use crate::types::types::Type;

pub struct StringValue {
    pub contents: String,
}

impl Data for StringValue {
    fn get_type(&self) -> Type {
        Type::String
    }

    fn print(&self) {
        print!("{0}", self.contents);
    }

    fn as_string(&self) -> &StringValue {
        &self
    }
}