

use super::{function::{function::Function, call::Call}, statement::Statement, expression::{Expression, ExpressionType}};

pub trait CheckCalls {
    fn check_for_calls(&self, functions : &mut Vec<Function>);
}

fn register_call(call : &Call, functions : &mut Vec<Function>) {
    for expr in &call.args {
        expr.check_for_calls(functions)
    }

    let mut found = false;

    for func in functions {
        if call.is_match(&func) {
            // match!
            func.used = true;
            found = true;
            break;
        }
    }    

    assert!(found, "No matching function found for {:?}", call);
}

impl CheckCalls for Statement {
    fn check_for_calls(&self, functions : &mut Vec<Function>) {
        match self {
            Statement::Expression(expr) 
            | Statement::Assignment(_, expr) => expr.check_for_calls(functions),
        }
    }
}

impl CheckCalls for Expression {
    fn check_for_calls(&self, functions : &mut Vec<Function>) {
        match &self.expr_type {
            ExpressionType::FunctionCall(call) => {
                register_call(&call, functions)
            },
            ExpressionType::Literal(_) | ExpressionType::VariableRead(_) => {},
        }
    }
}

