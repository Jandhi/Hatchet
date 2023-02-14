use crate::functions::args::Args;
use crate::scope::Scope;
use crate::functions::function::Function;
use crate::value::Value;
use crate::none::None;
use std::collections::HashMap;

fn print(args: Args) -> Box< dyn Value> {
    let _ = args.contents[0].value.print();
    Box::from(None{})
}

pub fn load_base_scope() -> Scope {
    let base_functions : HashMap<String, Box<dyn Value>> = HashMap::from([
        (String::from("print"), Box::from(Function{
            func: print,
            min_args: 1,
            max_args: 1,
        }) as Box<dyn Value>)
    ]);

    Scope {
        items : base_functions
    }
}
