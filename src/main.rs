use std::{fs::File, io::Write, env};

use read::read_file;

use crate::parser::{program::{CodeWriter}, parse::Parser, context::{make_empty_context}};

pub mod lexer {
    pub mod lexemes;

}
pub mod literal;
pub mod read;
pub mod stdlib {
    pub mod stdlib;
    pub mod print;
    pub mod echo;
    pub mod math {
        pub mod add;
        pub mod sub;
    }
    pub mod list {
        pub mod list;
    }
}
pub mod parser {
    pub mod program;
    pub mod statement;
    pub mod call_checker;
    pub mod parse;
    pub mod expression;
    pub mod assignment;
    pub mod context;
    pub mod variables;
    pub mod function {
        pub mod call;
        pub mod function;
    }
}
pub mod my_types;
pub mod types {
    pub mod type_checker;
    pub mod hatchet_type;
    pub mod composite_type;
    pub mod primitive_type;
    pub mod union_type;
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let mut file_name = String::from("tests/test4.hat");

    if args.len() > 1 {
        file_name = args[1].clone();
    }

    let mut my_parser = Parser{ functions: vec![], main: vec![] };
    stdlib::stdlib::load(&mut my_parser);

    let mut lexed = read_file(file_name);
    println!("{:?}", lexed);
    let mut buffer = String::from("");
    let parsed = my_parser.parse(&mut lexed);

    println!("{:?}", parsed.main);

    parsed.write(&mut buffer, &mut make_empty_context());
    println!("{}", buffer);

    let mut f = File::create("out.cpp").expect("");
    let _ = f.write(buffer.as_bytes());

    println!("----- END OF PARSE -----\n")
}