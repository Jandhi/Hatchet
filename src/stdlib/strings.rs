use std::collections::HashMap;

use crate::{value::Value, function::{Function, FunctionType::BuiltIn, ParameterAmount::Unlimited, Args}, state::State};



fn make_print() -> Value {
    fn print(args : Args) -> Value {
        for arg in args {
            print!("{} ", arg);
        }
    
        println!();
    
        Value::None
    }

    return Value::Function(Function {
        func_type: BuiltIn(print),
        param_amt: Unlimited  
    })
}

pub fn load(state : &mut State) {
    state.scopes.push(crate::scope::Scope { 
        name: String::from("std_strings"), 
        operators: HashMap::from([
        ]),
        identifiers: HashMap::from([
            (String::from("print"), make_print()),
        ]), 
        arguments: vec![],
    })
}