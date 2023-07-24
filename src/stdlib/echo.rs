use std::{rc::Rc, vec};
use crate::parser::function::function::{Function, get_arg_name};
use crate::parser::function::function::FunctionDefinition::Predefined;
use crate::types::primitive_type::STRING_TYPE;

pub fn make_echo() -> Function {
    Function{
        name: Rc::from("echo"),
        arguments: vec![STRING_TYPE],
        definition: Predefined(Rc::from(format!("return {};\n", get_arg_name(0)))),
        used: false,
        return_type: STRING_TYPE,
    }
}

