use std::env;

use hatchet::{input::read_file, lexer::{lexer::Lexer, token::Token}, parser::{expression::Expression, parser::Parser, node::NodeDisplay}};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    println!("LEXING");

    if args.len() < 2 {
        args.push("tests/expr_test.hat".to_string());
    }

    let content = read_file(&args[1]);
    let res_lexemes = Lexer::process(&content);

    if let Err(error) = res_lexemes {
        println!("Lex Error {:?} at {:?}: {}", error.error, error.position, error.description);
        return;
    }

    let tokens = res_lexemes.unwrap();

    for token in tokens.iter() {
        println!("{:?}", token.token_type);
    }
    let mut token_refs = tokens.iter().collect();

    println!("PARSING");

    let res_parsed = Expression::parse(&mut token_refs, ());

    if let Err(error) = res_parsed {
        println!("Parse Error {:?} at {:?}: {}", error.error, error.position, error.description);
        return;
    }
    let parsed = res_parsed.unwrap();

    parsed.node_display(0);
}
