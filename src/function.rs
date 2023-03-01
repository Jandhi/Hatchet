use std::{collections::HashMap, rc::Rc};

use crate::{value::Value, expression::{Expression, evaluate}, state::State, scope::Scope};

#[derive(Clone)]
pub struct Function {
    pub func_type : FunctionType,
    pub param_amt : ParameterAmount
}

pub enum FunctionType {
    BuiltIn(fn(Args) -> Value),
    Procedure(Procedure),
}

impl Clone for FunctionType {
    fn clone(&self) -> Self {
        match self {
            Self::BuiltIn(arg0) => Self::BuiltIn(arg0.clone()),
            Self::Procedure(arg0) => todo!("Procedures can't be cloned")
        }
    }
}


#[derive(Debug, Clone)]
pub enum ParameterAmount {
    Exact { amount : u8 },
    Range { min : u8 , max : u8 },
    Unlimited
}

pub type Args = Vec<Value>;
pub type Procedure = Vec<Expression>;

pub fn evaluate_function(func : &Function, args : Args, state : &mut State) -> Value {
    match &func.func_type {
        FunctionType::BuiltIn(function) => return function(args),
        FunctionType::Procedure(proc) => {
            state.scopes.push(Scope { identifiers: HashMap::new(), arguments: args });
            let mut val = Value::None;

            for expr in proc {
                val = evaluate(expr, state)
            }

            state.scopes.pop();
            return Value::None;
        }
    }
}

