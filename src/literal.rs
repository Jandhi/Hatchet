use crate::{my_types::Text, parser::program::{Writer, Program}, types::{hatchet_type::HasType, primitive_type::STRING_TYPE}};


#[derive(Debug, Clone)]
pub enum Literal {
    String(Text),
    Int32(i32),
    UInt32(u32),
}

impl Writer for Literal {
    fn write(&self, buffer : &mut String, program : &Program) {
        match self {
            Literal::String(text) => {
                buffer.push('\"');
                buffer.push_str(text);
                buffer.push('\"');
            },
            _ => todo!()
        };
    }
}

impl HasType for Literal {
    fn get_type(&self, program : &Program) -> crate::types::hatchet_type::HatchetType {
        match &self {
            Literal::String(_) => STRING_TYPE,
            Literal::Int32(_) => todo!(),
            Literal::UInt32(_) => todo!(),
        }
    }
}