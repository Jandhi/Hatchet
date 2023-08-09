use std::{rc::Rc, vec};
use crate::parser::function::function::{Function, get_arg_name};
use crate::parser::function::function::FunctionDefinition::Predefined;
use crate::parser::parse::Parser;
use crate::types::primitive_type::{INT_TYPE};

pub fn load_sub(parser : &mut Parser) {
    parser.functions.push(Function{
        name: Rc::from("sub"),
        arguments: vec![INT_TYPE, INT_TYPE],
        definition: Predefined(Rc::from(format!("return {} - {};\n", get_arg_name(0), get_arg_name(1)))),
        used: false,
        return_type: INT_TYPE,
    });
}

