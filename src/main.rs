mod expression;
mod function;
mod parsing {
    pub mod parsing_error;
    pub mod token;
    pub mod tokenizer;
}
mod stdlib {
    pub mod load;
    pub mod print;
}
mod translation {
    pub mod translate;
    pub mod translation_error;
}
mod scope;
mod value;
mod state;

use std::fs;
use expression::evaluate;
use parsing::parsing_error::print_parsing_error;
use state::State;
use stdlib::load::load_stdlib;
use translation::translate::translate;

use crate::parsing::tokenizer::tokenize;

fn main() {

    let contents = fs::read_to_string("hatchet/simple/1_hello_world.hat")
        .expect("Should have been able to read the file");

    let parse_result = tokenize(contents);

    if parse_result.is_err() {
        print_parsing_error(parse_result.unwrap_err());
        return;
    }

    let mut tokens = parse_result.unwrap();

    let expr = translate(&mut tokens).unwrap();

    let mut state = State{
        scopes: vec![load_stdlib()]
    };

    evaluate(&expr, &mut state);

}
