use std::vec;

use crate::lexer::token::Token;
use crate::lexer::token::TokenType;

const KEYWORDS: &'static [&'static str] = &[
    "if", 
    "while"
];

pub struct LexingError {
    message : String
}

pub fn lex(mut contents : &str) -> Result<Vec<Token>, LexingError> {
    let mut line_num = 0;
    let mut character_num = 0;
    let mut tokens : Vec<Token> = vec![];

    while contents.len() > 0 {
        if contents.starts_with('\'') {
            contents = &contents[1..];
            character_num += 1;
    
            let end_index = contents.find('\'');
    
            if end_index.is_none() {
                return Err(LexingError { message : String::from("Closing \' not found") });
            }
    
            let string_value = &contents[..(end_index.unwrap())];
            contents = &contents[end_index.unwrap()..];

            tokens.push(Token {
                token_type : TokenType::Literal,
                contents : String::from(string_value)
            });
        }
        
        if contents.chars().next().unwrap().is_alphabetic() {
            let end_index = contents.chars().position(|c| !c.is_alphabetic());

            let identifier_value = &contents[..end_index.unwrap()];
            contents = &contents[end_index.unwrap()..];

            tokens.push(Token {
                token_type : TokenType::Identifier,
                contents : String::from(identifier_value)
            });
        }

        if contents.starts_with(' ')
        {
            contents = &contents[1..];
        }
    }

    return Ok(tokens);
}