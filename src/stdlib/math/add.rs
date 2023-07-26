use std::{rc::Rc, vec};
use crate::parser::function::function::{Function, get_arg_name};
use crate::parser::function::function::FunctionDefinition::Predefined;
use crate::parser::parse::Parser;
use crate::types::primitive_type::{STRING_TYPE, INT_TYPE};

pub fn load_add(parser : &mut Parser) {
    parser.functions.push(Function{
        name: Rc::from("add"),
        arguments: vec![STRING_TYPE, STRING_TYPE],
        definition: Predefined(Rc::from(format!("return {} + {};\n", get_arg_name(0), get_arg_name(1)))),
        used: false,
        return_type: STRING_TYPE,
    });

    parser.functions.push(Function{
        name: Rc::from("add"),
        arguments: vec![INT_TYPE, INT_TYPE],
        definition: Predefined(Rc::from(format!("return {} + {};\n", get_arg_name(0), get_arg_name(1)))),
        used: false,
        return_type: INT_TYPE,
    });
}

