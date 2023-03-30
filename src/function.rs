use std::{collections::HashMap, rc::Rc};

use crate::{value::Value, expression::{Expression, evaluate}, state::State, scope::Scope};

#[derive(Clone)]
pub struct Function {
    pub name : String,
    pub func_type : FunctionType,
    pub params : Vec<String>,
}

pub enum FunctionType {
    BuiltIn(fn(Args) -> Value),
    Defined(Expression),
}

impl Clone for FunctionType {
    fn clone(&self) -> Self {
        match self {
            Self::BuiltIn(func) => Self::BuiltIn(func.clone()),
            Self::Defined(expressions) => todo!("Procedures can't be cloned")
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

pub fn evaluate_function(func : &Function, args : Args, state : &mut State) -> Value {
    match &func.func_type {
        FunctionType::BuiltIn(function) => return function(args),
        FunctionType::Defined(expr) => {
            let mut scope = Scope { name : String::from("Function"), identifiers: HashMap::new(), operators: HashMap::new(), arguments: args };

            
            for param in &func.params {
                let arg = scope.arguments.pop().unwrap();
                scope.identifiers.insert(param.clone(), arg);    
            }

            state.scopes.push(scope);
            evaluate(expr, state)
        }
    }
}

