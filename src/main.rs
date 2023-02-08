use base_functions::load_base_scope;
use functions::function::Function;
use types::string::StringValue;
use std::fs;

mod value;
mod types { 
    pub mod types; 
    pub mod string; 
    pub mod integer;
}
mod scope;
mod base_functions;
mod functions { 
    pub mod function; 
    pub mod function_call;
}
mod none;
mod expression;
mod lexer {
    pub mod lexer;
    pub mod token;
}

fn main() {
    let scope = load_base_scope();

    let func = (*scope.items[&String::from("print")]).as_func();

    let method = func.func;

    let retval = (method)(&vec![Box::from(StringValue{contents: String::from("test")})]);

    let contents = fs::read_to_string("test.hat")
        .expect("Should have been able to read the file");

    print!("{}", contents);
}
