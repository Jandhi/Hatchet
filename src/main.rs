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

use std::{fs};
use colored::Colorize;
use expression::evaluate;
use parsing::parsing_error::print_parsing_error;
use state::State;
use stdlib::load::load_stdlib;
use translation::{translate::translate, translation_error::print_translation_error};

use crate::parsing::tokenizer::tokenize;

fn main() {
    run_all();
    // run_file(String::from("hatchet/tests/12_pipe.hat"));
}

fn run_all() {
    for name in vec![
        "1_hello",
        "2_brackets",
        "3_operators",
        "4_hey",
        "5_ints",
        "6_no_bedmas",
        "7_lines",
        "8_bools",
        "9_and",
        "10_lines2",
        "11_or",
        "12_pipe",
    ] {
        run_file(format!("hatchet/tests/{}.hat", name));
        println!();
    }
}

fn run_file(name : String) {
    let title = format!("RUNNING {}", name).red();
    println!("{}", title);

    let mut state = State{
        scopes: vec![]
    };

    load_stdlib(&mut state);

    let contents = fs::read_to_string(name)
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
    

    let translate_result = translate(&mut tokens);

    match translate_result {
        Ok(expr) => {
            println!("{}", "--------".blue());
            println!("{}", "PROCEDURE:".blue());
            println!("{}", expr.expr_type.to_string());

            println!("{}", "--------".blue());
            println!("{}", "OUTPUT:".blue());
            evaluate(&expr, &mut state);
        },
        Err(err) => print_translation_error(err),
    }
}