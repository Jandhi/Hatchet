use std::collections::HashMap;
use crate::{value::Value};

pub struct  Scope {
    pub name : String,
    pub identifiers : HashMap<String, Value>,
    pub operators : HashMap<String, Value>,
    pub arguments : Vec<Value>,
}