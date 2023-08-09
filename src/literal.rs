use crate::{parser::{program::{CodeWriter}, context::Context}, my_types::Text};


#[derive(Debug, Clone)]
pub enum Literal {
    String(Text),
    Int32(i32),
    UInt32(u32),
}

impl CodeWriter for Literal {
    fn write(&self, buffer : &mut String,_context : &mut Context) {
        match self {
            Literal::String(text) => {
                buffer.push('\"');
                buffer.push_str(text);
                buffer.push('\"');
            },
            Literal::Int32(text) => {
                buffer.push_str(format!("{}", text).as_str());
            },
            _ => todo!()
        };
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