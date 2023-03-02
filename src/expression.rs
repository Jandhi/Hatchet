use crate::{function::{evaluate_function, Procedure}, value::Value, state::State, parsing::{tokenizer::ParserPosition}};

pub struct Expression {
    pub position : ParserPosition,
    pub expr_type : ExpressionType
}

pub enum ExpressionType {
    Reference(String),
    Value(Value),
    FunctionCall(Box<Expression>, Vec<Expression>),
    Procedure(Procedure),
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
    }
}

pub fn evaluate_function_call(callee : &Value, args : &Vec<Expression>, state : &mut State) -> Value {
    match callee {
        Value::Function(func) => {
            let values : Vec<Value> = args.into_iter().map(|exp| evaluate(exp, state)).collect();
            evaluate_function(&func, values, state)
        }
        
        _ => {
            todo!("Callable error")
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