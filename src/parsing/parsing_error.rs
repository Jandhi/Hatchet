use colored::Colorize;

use super::tokenizer::ParserPosition;

#[derive(Debug)]
pub struct ParsingError {
    pub position : ParserPosition,
    pub err_type : ErrorType
}

#[derive(Debug)]
pub enum ErrorType {
    MisplacedQuote,
    MissingClosingQuote,
    UnexpectedCharacter(char),
}

pub fn print_parsing_error(error : ParsingError) {
    print!("{} at line {} col {}: ", "ERROR".red(), error.position.line, error.position.column);

    match error.err_type {
        ErrorType::MisplacedQuote => println!("Missplaced Quote"),
        ErrorType::MissingClosingQuote => println!("Missing Closing Quote"),
        ErrorType::UnexpectedCharacter(char) => println!("Unexpected Character: '{}'", char)
    }
}