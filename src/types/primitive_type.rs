use std::rc::Rc;

use crate::{my_types::Text, parser::{program::{CodeWriter}, context::Context}};

use super::hatchet_type::HatchetType;

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    String,
    Int,
    List(Rc<HatchetType>),
}

pub const STRING_TYPE: HatchetType = HatchetType::Primitive(PrimitiveType::String);
pub const INT_TYPE: HatchetType = HatchetType::Primitive(PrimitiveType::Int);

impl PrimitiveType {
    pub fn get_name(&self) -> Text {
        match self {
            PrimitiveType::String => Rc::from("String") ,
            PrimitiveType::Int => Rc::from("Int"),
            PrimitiveType::List(subtype) => Rc::from(format!("List<{}>", subtype.get_name())),
        }
    }
}

impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PrimitiveType::String, PrimitiveType::String) => true,
            (PrimitiveType::Int, PrimitiveType::Int) => true,
            _ => false,
        }
    }
}

impl CodeWriter for PrimitiveType {
    fn write(&self, buffer : &mut String, context : &mut Context) {
        match self {
            PrimitiveType::String => buffer.push_str("std::string"),
            PrimitiveType::Int => buffer.push_str("int"),
            PrimitiveType::List(subtype) => {
                buffer.push_str("std::vector<");
                subtype.write(buffer, context);
                buffer.push_str(">");
            },
        }
    }
}