use crate::{parser::{constant::Constant, node::Node}, types::hatchet_type::{TypeData, Type, BaseType}};

use super::typechecker::{TypeCheckable, TypeContext};

impl TypeCheckable for Node<Constant> {
    fn type_check(&mut self, ctx : &mut TypeContext) -> &TypeData {
        self.type_data.mutable = false;
        self.type_data.my_type = match self.content {
            Constant::Int(_) => Type::Base(BaseType::Int),
            Constant::String(_) => Type::Base(BaseType::String),
        };

        &self.type_data
    }
}