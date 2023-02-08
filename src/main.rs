use base_functions::load_base_scope;
use functions::function::Function;
use types::string::StringValue;
use std::fs;
use lexer::lexer::lex;

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
mod parser {
    pub mod parse;
}
mod block;

fn main() {

    let contents = fs::read_to_string("hatchet/test.hat")
        .expect("Should have been able to read the file");

    lex(contents.as_str());
}
