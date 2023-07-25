use std::{fs::File, io::Write};

use read::read_file;

use crate::parser::{program::{CodeWriter}, parse::Parser, context::{make_context}};

pub mod lexer {
    pub mod lexemes;

}
pub mod literal;
pub mod read;
pub mod stdlib {
    pub mod stdlib;
    pub mod print;
    pub mod echo;
}
pub mod parser {
    pub mod program;
    pub mod statement;
    pub mod call_checker;
    pub mod parse;
    pub mod expression;
    pub mod assignment;
    pub mod context;
    pub mod function {
        pub mod call;
        pub mod function;
    }
}
pub mod my_types;
pub mod types {
    pub mod hatchet_type;
    pub mod composite_type;
    pub mod primitive_type;
    pub mod union_type;
}

fn main() {
    let mut my_parser = Parser{ functions: vec![], main: vec![] };
    stdlib::stdlib::load(&mut my_parser);

    let mut lexed = read_file(String::from("tests/test2.hat"));
    println!("{:?}", lexed);
    let mut buffer = String::from("");
    let parsed = my_parser.parse(&mut lexed);

    println!("{:?}", parsed.main);

    parsed.write(&mut buffer, &parsed, &make_context());
    println!("{}", buffer);

    let mut f = File::create("out.cpp").expect("");
    let _ = f.write(buffer.as_bytes());
}