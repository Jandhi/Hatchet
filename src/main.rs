use base_functions::load_base_scope;
use functions::function::Function;
use types::string::StringValue;


mod value;
mod types { 
    pub mod types; 
    pub mod string; 
    pub mod integer;
}
mod scope;
mod base_functions;
mod functions { pub mod function; }
mod none;

fn main() {
    let scope = load_base_scope();

    let func = (*scope.items[&String::from("print")]).as_func();

    let method = func.func;

    let retval = (method)(vec![Box::from(StringValue{contents: String::from("test")})]);
}
