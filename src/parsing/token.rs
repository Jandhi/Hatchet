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
    StringLiteral(String),
    IntegerLiteral(IntegerVal),
    OpenParentheses,
    CloseParentheses,
    Operator(String),
}