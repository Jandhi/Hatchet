use std::fmt::Debug;

use crate::position::Position;


#[derive(Clone)]
pub struct Lexeme {
    pub position : Position,
    pub lex_type : LexemeType
}

impl Debug for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.lex_type)
    }
}

#[derive(Debug, Clone)]
pub enum LexemeType {
    Identifier(Identifier),
    KeyWord(KeyWord),
    Separator(Separator),
    Operator(Operator),
    Literal(Literal)
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name : String
}

impl Into<LexemeType> for Identifier {
    fn into(self) -> LexemeType {
        LexemeType::Identifier(self)
    }
}

#[derive(Debug, Clone)]
pub enum KeyWord {
    
}

impl Into<LexemeType> for KeyWord {
    fn into(self) -> LexemeType {
        LexemeType::KeyWord(self)
    }
}

#[derive(Debug, Clone)]
pub enum Separator {
    LeftParen,
    RightParen
}

impl Into<LexemeType> for Separator {
    fn into(self) -> LexemeType {
        LexemeType::Separator(self)
    }
}

#[derive(Debug, Clone)]
pub enum Operator {

}

impl Into<LexemeType> for Operator {
    fn into(self) -> LexemeType {
        LexemeType::Operator(self)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Integer(i32),
}

impl Into<LexemeType> for Literal {
    fn into(self) -> LexemeType {
        LexemeType::Literal(self)
    }
}