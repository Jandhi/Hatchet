use std::collections::HashMap;

use crate::value::Value;


pub struct Args {
    pub contents : Vec<Argument>
}

impl Clone for Args {
    fn clone(&self) -> Self {
        Self { contents: self.contents[..]. }
    }
}


pub struct Argument {
    pub name : String,
    pub value : Box<dyn Value>
}   