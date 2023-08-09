use std::ops::Deref;

use crate::{my_types::Text, types::hatchet_type::HatchetType};

use super::{context::Context, program::CodeWriter};

#[derive(Clone)]
pub struct Variable {
    pub name : Text,
    pub htype : HatchetType,
    pub is_constant : bool
}

pub fn find_variable<'a>(name : &Text, context : &'a Context<'a, 'a>) -> &'a Variable {
    for variable in &context.variables {
        if variable.name.deref() == name.deref() {
            return variable
        }
    }

    panic!("Variable {} not found!", name)
}

impl CodeWriter for Variable {
    fn write(&self, buffer : &mut String, _context : &mut Context) {
        buffer.push_str("var_");
        buffer.push_str(&self.name)
    }
}