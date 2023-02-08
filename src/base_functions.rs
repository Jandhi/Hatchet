use crate::scope::Scope;
use crate::functions::function::Function;
use crate::value::Value;
use crate::none::None;
use std::collections::HashMap;

fn print(args: &Vec<Box<dyn Value>>) -> Box<dyn Value> {
    let _ = &args[0].print();
    return Box::from(None{});
}

pub fn load_base_scope() -> Scope {
    let func : Box<dyn Value> = Box::from(Function{
        func: print,
        min_args: 1,
        max_args: 1,
    });

    let base_functions : HashMap<String, Box<dyn Value>> = HashMap::from([
        (String::from("print"), func)
    ]);

    Scope {
        items : base_functions
    }
}
