use std::collections::HashMap;

use crate::scope::Scope;

use super::print::make_print;

pub fn load_stdlib() -> Scope {
    Scope {
        identifiers: HashMap::from([
            (String::from("print"), make_print()),
        ]),
        arguments: vec![],
    }
}