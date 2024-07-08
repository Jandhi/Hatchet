use std::rc::Rc;

use super::{pointer::Pointer, structure::Struct};

#[derive(Debug, Clone)]
pub struct TypeData {
    pub my_type : Type,
    pub mutable : bool,
}

impl TypeData {
    pub fn unknown() -> TypeData {
        TypeData { my_type: Type::Unknown, mutable: true }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Unknown,
    Base(BaseType),
    Ptr(Pointer),
    Struct(Rc<Struct>)
}

#[derive(Debug, Clone)]
pub enum BaseType {
    Int,
    String,
}