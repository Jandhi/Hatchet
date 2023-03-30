use std::collections::HashMap;

use crate::{value::Value, function::{Function, FunctionType::BuiltIn, ParameterAmount::Unlimited, Args}, state::State};

fn make_and() -> Value {
    fn print(args : Args) -> Value {
        if let Value::Boolean(val1) = args[0] {
            if let Value::Boolean(val2) = args[1] {
                return Value::Boolean(val1 && val2);
            }
        }

        panic!("You cannot use and on {} and {}", args[0], args[1]);
    }

    return Value::Function(Box::from(Function {
        name: String::from("and"), 
        func_type: BuiltIn(print),
        params: vec![],
    }))
}

fn make_or() -> Value {
    fn print(args : Args) -> Value {
        if let Value::Boolean(val1) = args[0] {
            if let Value::Boolean(val2) = args[1] {
                return Value::Boolean(val1 || val2);
            }
        }

        panic!("You cannot use or on {} and {}", args[0], args[1]);
    }

    return Value::Function(Box::from(Function {
        name: String::from("or"),
        func_type: BuiltIn(print),
        params: vec![],
    }))
}

pub fn load(state : &mut State) {
    state.scopes.push(crate::scope::Scope { 
        name: String::from("std_strings"), 
        operators: HashMap::from([
            (String::from("and"), make_and()),
            (String::from("or"), make_or()),
        ]),
        identifiers: HashMap::from([
           
        ]), 
        arguments: vec![],
    })
}