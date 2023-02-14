use std::collections::HashMap;
use crate::value::Value;

pub struct Scope {
    pub items : HashMap<String, Box<dyn Value>>,
}