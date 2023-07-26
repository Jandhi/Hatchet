
use std::{rc::Rc};

use crate::{my_types::Text, parser::{program::{CodeWriter, Program}, context::Context}};

use super::{primitive_type::PrimitiveType, union_type::UnionType, composite_type::CompositeType};

#[derive(Debug, Clone)]
pub enum HatchetType {
    Primitive(PrimitiveType),
    Union(Rc<UnionType>),
    Composite(Rc<CompositeType>),
    Constant(Rc<HatchetType>),
    Pointer(Rc<HatchetType>),
    Function(Rc<FunctionType>),
    None,
    Unknown
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub return_type : HatchetType,
    pub args : Vec<HatchetType>,
}

impl FunctionType {
    pub fn signature(&self) -> String {
        let mut arg_str = String::from("");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                arg_str.push_str(", ");
            }

            arg_str.push_str(&arg.get_name());
        }

        format!("fn ({}) -> {}", arg_str, self.return_type.get_name())
    }

    pub fn get_cpp_name(&self) -> String {
        let mut arg_string = String::from("");

        for (i, arg) in self.args.iter().enumerate() {
            if(i > 0) {
                arg_string.push_str("_")
            }

            arg_string.push_str(&arg.get_name());
        }

        return format!("fntype_{}_to_{}", arg_string, self.return_type.get_name())
    }
}

impl HatchetType {
    pub fn get_name(&self) -> Text {
        match self {
            HatchetType::Primitive(primitive) => primitive.get_name(),
            HatchetType::Union(union) => Rc::clone(&union.name),
            HatchetType::Composite(composite) => Rc::clone(&composite.name),
            HatchetType::Constant(inner) => inner.get_name(),
            HatchetType::Pointer(inner) => Rc::from(format!("{}*", inner.get_name())),
            HatchetType::None => Rc::from("None"),
            HatchetType::Unknown => Rc::from("UNKNOWN?"),
            HatchetType::Function(func) => Rc::from(func.signature().as_str()),
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

impl CodeWriter for HatchetType {
    fn write(&self, buffer : &mut String, context : &mut Context) {
        match self {
            HatchetType::Primitive(primitive) => primitive.write(buffer, context),
            HatchetType::Union(_) => todo!(),
            HatchetType::Composite(_) => todo!(),
            HatchetType::Constant(_) => todo!(),
            HatchetType::Pointer(inner) => {
                inner.write(buffer, context);
                buffer.push('*');
            },
            HatchetType::None => buffer.push_str("void"),
            HatchetType::Unknown => panic!("Type is not known!"),
            HatchetType::Function(func) => {todo!()},
        };
    }
}

