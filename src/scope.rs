use std::collections::HashMap;
use crate::value::Data;

pub struct Scope {
    pub items : HashMap<String, Box<dyn Data>>,
}