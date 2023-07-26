

use std::ops::Deref;

use crate::{parser::{expression::Expression, program::{CodeWriter}, context::Context}, my_types::Text, types::{type_checker::HasType, hatchet_type::HatchetType}};

use super::function::{Function, get_function_cpp_name};

#[derive(Debug)]
pub struct Call {
    pub func_name : Text,
    pub args : Vec<Expression>,
    pub my_type : HatchetType,
}

impl Call {
    pub fn signature(&self) -> String {
        let mut my_str = String::from("");

        my_str.push_str(&self.func_name);
        my_str.push('(');
        
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                my_str.push_str(", ");
            }

            my_str.push_str(arg.get_type().get_name().deref());
        }
        
        my_str.push(')');
        my_str
    }

    pub fn is_match(&self, function : &Function) -> bool {
        if function.name != self.func_name {
            return false
        }

        if self.args.len() != function.arguments.len() {
            return false
        }

        for i in 0..self.args.len() {
            if !self.args[i].get_type().is_type(&function.arguments[i]) {
                return  false;
            }
        }

        true
    }

    fn find_matching_func<'a, 'b>(&self, context : &'a mut Context) -> Option<&'b Function> where 'a : 'b {
        for func in &context.functions {
            if self.is_match(&func) {
                return Some(func);
            }
        }
        
        None
    }

    pub fn get_func<'a, 'b>(&self, context : &'a mut Context) -> &'b Function where 'a : 'b {
        match self.find_matching_func(context) {
            Some(func) => func,
            None => panic!("Didn't find matching function for call {:?}", self),
        }
    }
}

impl CodeWriter for Call {
    fn write(&self, buffer : &mut String, context : &mut Context) {
        let _func = self.find_matching_func( context);

        buffer.push_str(&get_function_cpp_name(self.func_name.clone()));
        buffer.push_str("(");

        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                buffer.push_str(", ");
            }

            arg.write(buffer, context);   
        }

        buffer.push_str(")");
    }
}