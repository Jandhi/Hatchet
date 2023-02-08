use crate::scope::Scope;
use crate::functions::function::Function;
use crate::value::Data;
use crate::value::Value;
use crate::none::None;
use std::collections::HashMap;

fn print(args: &Vec<Value>) -> Value {
    let _ = &args[0].data.print();
    return Value{data : Box::from(None{})};
}

pub fn load_base_scope() -> Scope {
    let base_functions : HashMap<String, Box<dyn Data>> = HashMap::from([
        (String::from("print"), Box::from(Function{
            func: print,
            min_args: 1,
            max_args: 1,
        }) as Box<dyn Data>)
    ]);

    Scope {
        items : base_functions
    }
}
