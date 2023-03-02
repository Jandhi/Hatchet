use crate::{value::Value, function::{Function, FunctionType::BuiltIn, ParameterAmount::Unlimited, Args}};

pub fn print(args : Args) -> Value {
    for arg in args {
        print!("{} ", arg);
    }

    println!();

    Value::None
}

pub fn make_print() -> Value {
    return Value::Function(Function {
        func_type: BuiltIn(print),
        param_amt: Unlimited  
    })
}