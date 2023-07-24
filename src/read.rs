use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::lexer::lexemes::{make_lexer, Lexeme};

pub fn read_file(file_name : String) -> Vec<Lexeme> {
    let reader = BufReader::new(File::open(file_name).expect("open failed"));

    let mut lexer = make_lexer();

    for line in reader.lines() {
        for c in line.expect("lines failed").chars() {
            lexer.consume(c);
        }

        lexer.consume('\n');
    }

    return lexer.lexemes;
}