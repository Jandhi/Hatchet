use crate::{value::Value, function::{Function, FunctionType::BuiltIn, ParameterAmount::Unlimited, Args}};

pub fn print_value(val : &Value) {
    match val {
        Value::String(str) => print!("{}", str),
        Value::Integer(int) => print!("{}", int),
        Value::None => print!("None"),
        _ => (),
    }
}

pub fn print(args : Args) -> Value {
    for arg in args {
        print_value(&arg)
    }

    Value::None
}

pub fn make_print() -> Value {
    return Value::Function(Function {
        func_type: BuiltIn(print),
        param_amt: Unlimited  
    })
}