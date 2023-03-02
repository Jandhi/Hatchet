use std::collections::HashMap;

use crate::scope::Scope;

use super::{print::make_print, operators::{make_add, make_sub, make_div, make_mult}};

pub fn load_stdlib() -> Scope {
    Scope {
        identifiers: HashMap::from([
            (String::from("print"), make_print()),
            (String::from("+"), make_add()),
            (String::from("-"), make_sub()),
            (String::from("*"), make_mult()),
            (String::from("/"), make_div()),
        ]),
        arguments: vec![],
    }
}