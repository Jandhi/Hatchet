use std::collections::HashMap;
use crate::{value::Value};

pub struct Scope {
    pub identifiers : HashMap<String, Value>,
    pub arguments : Vec<Value>
}