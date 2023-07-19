use read::read_file;

pub mod lexer {
    pub mod lexemes;
    pub mod literal;
}
mod read;

fn main() {
    println!("{:?}", read_file(String::from("tests/test1.hat")));
}