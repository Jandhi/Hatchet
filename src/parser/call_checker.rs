use super::{function::{function::Function, call::Call}, statement::Statement, expression::Expression};

pub trait CheckCalls {
    fn check_for_calls(&self, functions : &mut Vec<&mut Function>);
}

fn register_call(call : &Call, functions : &mut Vec<&mut Function>) {
    let mut found = false;

    for func in &mut *functions {
        let left : &str = &call.func_name;
        let right : &str = &func.name;
        if  left == right {
            // match!
            func.used = true;
            found = true;
            break;
        }
    }

    assert!(found, "No matching function found for {:?}", call);

    for expr in &call.args {
        expr.check_for_calls(functions)
    }
}

impl CheckCalls for Statement {
    fn check_for_calls(&self, functions : &mut Vec<&mut Function>) {
        match self {
            Statement::Expression(expr) 
            | Statement::Assignment(_, expr) => expr.check_for_calls(functions),
        }
    }
}

impl CheckCalls for Expression {
    fn check_for_calls(&self, functions : &mut Vec<&mut Function>) {
        match self {
            Expression::FunctionCall(call) => {
                register_call(call, functions)
            },
            Expression::Literal(_) => {},
        }
    }
}

