use std::rc::Rc;

use crate::parser::parse::Parser;

use super::{print::{load_print}, echo::make_echo, math::add::load_add};

pub fn load(parser : &mut Parser) {
    load_print(parser);
    load_add(parser);
    parser.functions.push(make_echo());
}