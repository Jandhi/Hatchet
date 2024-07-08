use std::{collections::HashMap, ops::Deref};

use crate::types::hatchet_type::TypeData;

pub struct TypeContext<'a> {
    pub type_table : HashMap<String, TypeData>,
    pub to_resolve : Vec<&'a mut dyn TypeCheckable>
}

pub trait TypeCheckable {
    fn type_check(&mut self, ctx : &mut TypeContext) -> &TypeData;
}

impl TypeContext<'_> {
    pub fn get_type(&self, identifier : &str) -> Option<TypeData> {
        if !self.type_table.contains_key(identifier) {
            return None;
        }

        Some(self.type_table.get(identifier).unwrap().clone())
    }
}
