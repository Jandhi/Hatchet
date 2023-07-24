
use std::{rc::Rc, iter::Product, ops::Deref};

use crate::{my_types::Text, parser::program::{Writer, self, Program}};

use super::{primitive_type::PrimitiveType, union_type::UnionType, composite_type::CompositeType};

#[derive(Debug, Clone)]
pub enum HatchetType {
    Primitive(PrimitiveType),
    Union(UnionType),
    Composite(CompositeType),
    Constant(Rc<HatchetType>),
    None
}



impl HatchetType {
    pub fn get_name(&self) -> Text {
        match self {
            HatchetType::Primitive(primitive) => primitive.get_name(),
            HatchetType::Union(union) => Rc::clone(&union.name),
            HatchetType::Composite(composite) => Rc::clone(&composite.name),
            HatchetType::Constant(constant) => constant.get_name(),
            HatchetType::None => Rc::from("None"),
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            HatchetType::Primitive(_) => true,
            HatchetType::Constant(constant) => constant.is_primitive(),
            _ => false,
        }
    }

    pub fn is_type(&self, other_type : &HatchetType) -> bool {
        match (&self, other_type) {
            (HatchetType::Primitive(prim1), HatchetType::Primitive(prim2)) => {
                return prim1.eq(prim2);
            },
            (HatchetType::Union(_), HatchetType::Union(_)) => todo!(),
            (HatchetType::Composite(_), HatchetType::Composite(_)) => todo!(),
            (HatchetType::Constant(const1), HatchetType::Constant(const2)) => {
                return const1.is_type(const2);
            },
            (_, _) => false
        }
    }
}

impl Writer for HatchetType {
    fn write(&self, buffer : &mut String, program : &Program) {
        match self {
            HatchetType::Primitive(primitive) => primitive.write(buffer, program),
            HatchetType::Union(_) => todo!(),
            HatchetType::Composite(_) => todo!(),
            HatchetType::Constant(_) => todo!(),
            HatchetType::None => buffer.push_str("void"),
        };
    }
}

pub trait HasType {
    fn get_type(&self, program : &Program) -> HatchetType;
}