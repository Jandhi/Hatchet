use colored::Colorize;

use crate::parsing::{tokenizer::ParserPosition, token::TokenType};

#[derive(Debug)]
pub struct TranslationError {
    pub position : ParserPosition,
    pub err_type : ErrorType
}

#[derive(Debug)]
pub enum ErrorType {
    UnexpectedToken(TokenType),
    MissingClosingParentheses,
    EmptyPipe,
    EmptyExpression,
}


pub fn print_translation_error(error : TranslationError) {
    print!("{} at line {} col {}: ", "ERROR".red(), error.position.line, error.position.column);

    match error.err_type {
        ErrorType::UnexpectedToken(token_type) => {
            println!("Unexpected token type {:?}", token_type)
        }
        ErrorType::MissingClosingParentheses => {
            println!("Missing closing parentheses")
        }
        ErrorType::EmptyPipe => {
            println!("Missing second argument for pipe operator")
        }
        ErrorType::EmptyExpression => {
            println!("Expression is empty")
        },
    }
}