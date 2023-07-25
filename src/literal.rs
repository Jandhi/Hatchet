use crate::{parser::{program::{CodeWriter, Program}, context::WriterContext}, my_types::Text, types::{hatchet_type::HasType, primitive_type::STRING_TYPE}};


#[derive(Debug, Clone)]
pub enum Literal {
    String(Text),
    Int32(i32),
    UInt32(u32),
}

impl CodeWriter for Literal {
    fn write(&self, buffer : &mut String, program : &Program, context : &WriterContext) {
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

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Int32(l0), Self::Int32(r0)) => l0 == r0,
            (Self::UInt32(l0), Self::UInt32(r0)) => l0 == r0,
            _ => false,
        }
    }
}