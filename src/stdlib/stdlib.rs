use crate::parser::parse::Parser;

use super::{print::{load_print}, echo::make_echo};

pub fn load(parser : &mut Parser) {
    load_print(parser);
    parser.functions.push(make_echo());
}