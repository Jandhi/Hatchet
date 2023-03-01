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
}