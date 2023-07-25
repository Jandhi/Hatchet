use std::rc::Rc;

use crate::{my_types::Text, parser::{program::{CodeWriter, Program}, context::WriterContext}};

use super::hatchet_type::HatchetType;

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    String,
}

pub const STRING_TYPE: HatchetType = HatchetType::Primitive(PrimitiveType::String);

impl PrimitiveType {
    pub fn get_name(&self) -> Text {
        match self {
            PrimitiveType::String => Rc::from("String") ,
        }
    }
}

impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PrimitiveType::String, PrimitiveType::String) => true,
        }
    }
}

impl CodeWriter for PrimitiveType {
    fn write(&self, buffer : &mut String, _program : &Program, context : &WriterContext) {
        match self {
            PrimitiveType::String => buffer.push_str("const char*"),
        }
    }
}