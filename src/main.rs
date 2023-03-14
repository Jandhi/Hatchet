mod expression;
mod function;
mod parsing {
    pub mod parsing_error;
    pub mod token;
    pub mod tokenizer;
}
mod stdlib {
    pub mod load;
    pub mod strings;
    pub mod math;
    pub mod logic;
}
mod translation {
    pub mod translate;
    pub mod translation_error;
}
mod scope;
mod value;
mod state;

use std::fs;
use colored::Colorize;
use expression::evaluate;
use parsing::parsing_error::print_parsing_error;
use state::State;
use stdlib::load::load_stdlib;
use translation::{translate::translate, translation_error::print_translation_error};

use crate::parsing::tokenizer::tokenize;

fn main() {
    let mut state = State{
        scopes: vec![]
    };

    load_stdlib(&mut state);

    let contents = fs::read_to_string("hatchet/tests/6_no_bedmas.hat")
        .expect("Should have been able to read the file");


    let parse_result = tokenize(contents, &state);

    if parse_result.is_err() {
        print_parsing_error(parse_result.unwrap_err());
        return;
    }

    let mut tokens = parse_result.unwrap();

    println!("{}", "TOKENS:".blue());
    for token in tokens.iter() {
        println!("{:?}", token.token_type);
    }
    println!("{}", "--------".blue());
    println!("{}", "OUTPUT:".blue());

    let translate_result = translate(&mut tokens);

    match translate_result {
        Ok(expr) => {
            evaluate(&expr, &mut state);
        },
        Err(err) => print_translation_error(err),
    }
}
