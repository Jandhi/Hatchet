

use std::collections::HashMap;

use crate::{value::Value, function::{Function, FunctionType::BuiltIn, ParameterAmount}, state::State};

pub fn load(state : &mut State) {
    state.scopes.push(crate::scope::Scope { 
        name: String::from("std_math"), 
        operators: HashMap::from([
            (String::from("+"), make_add()),
            (String::from("-"), make_sub()),
            (String::from("*"), make_mult()),
            (String::from("/"), make_div()),
        ]),
        identifiers: HashMap::from([]), 
        arguments: vec![],
    })
}

fn make_add() -> Value {
    return Value::Function(Function {
        name: String::from("add"),
        func_type: BuiltIn(|args| {
            match &args[0] {
                Value::Integer(val1) => {
                    match &args[1] {
                        Value::Integer(val2) => Value::Integer(val1 + val2),
                        _ => panic!("You cannot add {} and {}", &args[0], &args[1])
                    }
                }
                Value::String(val1) => {
                    match &args[1] {
                        Value::String(val2) => {
                            let mut retval = String::new();
                            retval.push_str(val1);
                            retval.push_str(val2);
                            return Value::String(retval)
                        },
                        _ => panic!("You cannot add {} and {}", &args[0], &args[1])
                    }
                }
                _ => panic!("You cannot add {} and {}", &args[0], &args[1])
            }
        }),
        param_amt: ParameterAmount::Exact { amount: 2 }  
    })
}

fn make_sub() -> Value {
    return Value::Function(
        Function {
            name: String::from("sub"), 
            func_type: BuiltIn(|args| {
            if args.len() == 1 {
                match &args[0] {
                    Value::Integer(val) => return Value::Integer(-1 * val,),
                    _ => panic!("You cannot negate {}!", args[0])
                }
            } else {
                match args[0] {
                    Value::Integer(val1) => {
                        match args[1] {
                            Value::Integer(val2) => Value::Integer(val1 - val2),
                            _ => panic!("You cannot sub {} and {}", args[0], args[1])
                        }
                    }
                    _ => panic!("You cannot sub {} and {}", args[0], args[1])
                }
            }
        }), 
        param_amt: ParameterAmount::Range { min: 1, max: 2 } 
    })
}

fn make_mult() -> Value {
    return Value::Function(Function {
        name: String::from("mult"), 
        func_type: BuiltIn(|args| {
            match &args[0] {
                Value::Integer(val1) => {
                    match &args[1] {
                        Value::Integer(val2) => Value::Integer(val1 * val2),
                        _ => panic!("You cannot multiply {} and {}", &args[0], &args[1])
                    }
                }
                _ => panic!("You cannot multiply {} and {}", &args[0], &args[1])
            }
        }),
        param_amt: ParameterAmount::Exact { amount: 2 }  
    })
}

fn make_div() -> Value {
    return Value::Function(Function {
        name: String::from("div"), 
        func_type: BuiltIn(|args| {
            match &args[0] {
                Value::Integer(val1) => {
                    match &args[1] {
                        Value::Integer(val2) => Value::Integer(val1 / val2),
                        _ => panic!("You cannot divide {} and {}", &args[0], &args[1])
                    }
                }
                _ => panic!("You cannot divide {} and {}", &args[0], &args[1])
            }
        }),
        param_amt: ParameterAmount::Exact { amount: 2 }  
    })
}