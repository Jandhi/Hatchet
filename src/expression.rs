use crate::{function::{Function, evaluate_function, Procedure}, value::Value, state::State};
use std::collections::HashMap;

pub enum Expression {
    Reference(String),
    Value(Value),
    FunctionCall(Box<Expression>, Vec<Expression>),
    Procedure(Procedure),
}

pub fn evaluate(expr : &Expression, state : &mut State) -> Value {
    match expr {
        Expression::FunctionCall(func, args) => {
            let callee = &evaluate(func, state);
            evaluate_function_call(callee, args, state)
        }
        Expression::Procedure(proc) => {
            let mut ret_val = Value::None;

            for sub_expr in proc {
                ret_val = evaluate(sub_expr, state)
            }

            ret_val
        }
        Expression::Reference(name) => {
            evaluate_reference(name, state)
        }
        Expression::Value(value) => {
            return value.clone();
        }

        _ => todo!("Need to evaluate this expression: ")
    }
}

pub fn evaluate_function_call(callee : &Value, args : &Vec<Expression>, state : &mut State) -> Value {
    match callee {
        Value::Function(func) => {
            let values : Vec<Value> = args.into_iter().map(|exp| evaluate(exp, state)).collect();
            evaluate_function(&func, values, state)
        }
        
        _ => {
            todo!("ERROR")
        }
    }
}

pub fn evaluate_reference(name : &String, state : &mut State) -> Value {
    for scope in  &state.scopes {
        if scope.identifiers.contains_key(name) {
            return scope.identifiers[name].clone();
        }
    }

    Value::None
}