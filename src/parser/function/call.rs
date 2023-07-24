use std::rc::Rc;

use crate::{parser::{expression::Expression, program::{Writer, Program}}, my_types::Text, types::hatchet_type::HasType};

use super::function::{Function, get_function_cpp_name};

#[derive(Debug)]
pub struct Call {
    pub func_name : Text,
    pub args : Vec<Expression>,
}

impl Call {
    fn is_match(&self, program : &Program, function : &Function) -> bool {
        if function.name != self.func_name {
            return false
        }

        if self.args.len() != function.arguments.len() {
            return false
        }

        for i in 0..self.args.len() {
            if !self.args[i].get_type(program).is_type(&function.arguments[i]) {
                return  false;
            }
        }

        true
    }

    fn find_matching_func<'a, 'b>(&self, program : &'a Program) -> Option<&'b Function> where 'a : 'b {
        for func in &program.functions {
            if self.is_match(program, func) {
                return Some(func);
            }
        }
        
        None
    }

    pub fn get_func<'a, 'b>(&self, program : &'a Program) -> &'b Function where 'a : 'b {
        match self.find_matching_func(program) {
            Some(func) => func,
            None => panic!("Didn't find matching function for call {:?}", self),
        }
    }
}

impl Writer for Call {
    fn write(&self, buffer : &mut String, program : &Program) {
        let func = self.find_matching_func(program);

        buffer.push_str(&get_function_cpp_name(self.func_name.clone()));
        buffer.push_str("(");

        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                buffer.push_str(", ");
            }

            arg.write(buffer, program);   
        }

        buffer.push_str(")");
    }
}