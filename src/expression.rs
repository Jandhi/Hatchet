use core::panic;
use std::{thread::panicking, fmt::{Display, format}};

use crate::{function::{evaluate_function, Procedure}, value::Value, state::State, parsing::{tokenizer::ParserPosition}};

pub struct Expression {
    pub position : ParserPosition,
    pub expr_type : ExpressionType
}

pub enum ExpressionType {
    Reference(String),
    Value(Value),
    FunctionCall(Box<Expression>, Vec<Expression>),
    Pipe(Box<Expression>, Box<Expression>),
    Procedure(Procedure),
}

impl ExpressionType {
    pub fn to_string(&self) -> String {
        match self {
            ExpressionType::Reference(name) => format!("ref({})", name),
            ExpressionType::Value(val) => format!("value({})", val.to_string()),
            ExpressionType::FunctionCall(func, args) => {
                let mut arg_string = String::from("");

                for arg in args {
                    arg_string = format!("{}, {}", arg_string, arg.expr_type.to_string());
                }

                if arg_string.starts_with(", ") {
                    arg_string = String::from(&arg_string[2..]);
                }

                format!("call({})[{}]", func.expr_type.to_string(), arg_string)
            },
            ExpressionType::Pipe(bx1, bx2) => format!("pipe({}, {})", bx1.expr_type.to_string(), bx2.expr_type.to_string()),
            ExpressionType::Procedure(proc) => {
                let mut expr_string = String::from("");

                for expr in proc {
                    expr_string = format!("{}, {}", expr_string, expr.expr_type.to_string());
                }

                if expr_string.starts_with(", ") {
                    expr_string = String::from(&expr_string[2..]);
                }

                format!("proc({})", expr_string)
            }
        }
    }
}

pub fn evaluate(expr : &Expression, state : &mut State) -> Value {
    match &expr.expr_type {
        ExpressionType::FunctionCall(func, args) => {
            let callee = &evaluate(&func, state);
            evaluate_function_call(callee, &args, state)
        }
        ExpressionType::Procedure(proc) => {
            let mut ret_val = Value::None;

            for sub_expr in proc {
                ret_val = evaluate(&sub_expr, state)
            }

            ret_val
        }
        ExpressionType::Reference(name) => {
            evaluate_reference(&name, state)
        }
        ExpressionType::Value(value) => {
            return value.clone();
        }
        ExpressionType::Pipe(expr1, expr2) => {
            let arg0 = evaluate(expr1, state);

            if let ExpressionType::FunctionCall(callee, args) = &expr2.expr_type {
                let mut values : Vec<Value> = args.into_iter().map(|exp| evaluate(exp, state)).collect();
                values.insert(0, arg0);

                if let Value::Function(function) = evaluate(callee, state) {
                    return evaluate_function(&function, values, state);
                }

                panic!("Cannot call expression that isn't a function! at {}", callee.position);
                
            } else {
                let value = evaluate(expr2, state);

                match value {
                    Value::Function(func) => {
                        return evaluate_function(&func, vec![arg0], state)
                    },  
                    _ => {
                        panic!("Second part of pipe operator must be function or function call, not {}", value);
                    }
                }
            }
        },
    }
}

pub fn evaluate_function_call(callee : &Value, args : &Vec<Expression>, state : &mut State) -> Value {
    match callee {
        Value::Function(func) => {
            let values : Vec<Value> = args.into_iter().map(|exp| evaluate(exp, state)).collect();
            evaluate_function(&func, values, state)
        }
        
        _ => {
            todo!("Error, callee is {}", callee)
        }
    }
}

pub fn evaluate_reference(name : &String, state : &mut State) -> Value {
    for scope in  &state.scopes {
        if scope.operators.contains_key(name) {
            return scope.operators[name].clone();
        }

        if scope.identifiers.contains_key(name) {
            return scope.identifiers[name].clone();
        }
    }

    panic!("Unknown reference: \"{}\"", name);
}