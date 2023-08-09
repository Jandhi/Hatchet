use std::{rc::Rc, vec};
use crate::parser::function::function::{Function, get_arg_name};
use crate::parser::function::function::FunctionDefinition::Predefined;
use crate::parser::parse::Parser;
use crate::types::hatchet_type::HatchetType::None;
use crate::types::primitive_type::{STRING_TYPE, INT_TYPE};

pub fn load_list(parser : &mut Parser) {
    parser.functions.push(Function{
        name: Rc::from("print"),
        arguments: vec![STRING_TYPE],
        definition: Predefined(Rc::from(format!("std::cout << {} << std::endl;\n", get_arg_name(0)))),
        used: false,
        return_type: None,
    });
}

