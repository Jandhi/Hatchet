use std::fmt::Display;

use crate::position::Position;

use super::{keyword::KeyWord, literal::Literal, operator::Operator, separator::Separator};

#[derive(Debug)]
pub struct Token {
    pub token_type : TokenType,
    pub position : Position,
}

#[derive(Debug)]
pub enum TokenType {
    Identifier(String),
    KeyWord(KeyWord),
    Separator(Separator),
    Operator(Operator),
    Literal(Literal),
    Comment(String),
}

pub fn print_tokens(tokens : &Vec<&Token>) {
    for token in tokens.iter() {
        print!("{} ", token.token_type) 
    }
    println!();
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenType::Identifier(name) => format!("I({})", name),
            TokenType::KeyWord(keyword) => format!("{}", keyword),
            TokenType::Separator(sep) => format!("{}", sep),
            TokenType::Operator(op) => format!("{}", op),
            TokenType::Literal(literal) => format!("{}", literal),
            TokenType::Comment(_) => format!(""),
        };

        write!(f, "{}", str)
    }
}