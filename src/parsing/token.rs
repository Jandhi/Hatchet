use crate::value::IntegerVal;

use super::tokenizer::ParserPosition;

#[derive(Debug)]

pub struct Token {
    pub position : ParserPosition,
    pub token_type : TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    Identifier(String),
    Operator(String), // Special type of identifier that calls a function in between
    
    StringLiteral(String),
    IntegerLiteral(IntegerVal),
    Boolean(bool),
    
    OpenParentheses,
    CloseParentheses,
    
    Pipe,
    
    NewLine,
}