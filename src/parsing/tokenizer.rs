use std::vec;

use crate::{parsing::token::Token, state::State};
use super::{parsing_error::{ParsingError, ErrorType}, token::TokenType};

#[derive(Debug, Clone, Copy)]
pub struct ParserPosition {
    pub line : u16,
    pub column : u16
}

pub struct Parser<'a> {
    pub in_quotes : bool,
    pub tokens : Vec<Token>,
    pub buffer : String,
    pub expected_type : ExpectedType,
    pub position : ParserPosition,
    pub state : &'a State,
}

pub enum ExpectedType {
    Identifier,
    Integer,
    StringLiteral,
    Operator,
}

impl<'a> Parser<'a> {
    fn increment_col(&mut self) {
        self.position.column += 1;
    }

    fn send_buffer(&mut self) {
        if self.buffer.len() == 0 {
            return;
        }

        if self.state.contains_operator(&self.buffer) {
            self.expected_type = ExpectedType::Operator;
        }
        
        self.tokens.push(Token { 
            position: self.position.clone(),
            token_type: match self.expected_type {
                ExpectedType::Identifier => {
                    if self.buffer == "true" || self.buffer == "false" { // booleans
                        TokenType::Boolean(self.buffer == "true")
                    } else {
                        TokenType::Identifier(self.buffer.clone())
                    }
                },
                ExpectedType::Integer => TokenType::IntegerLiteral(self.buffer.clone().parse().expect("Couldn't parse integer!")),
                ExpectedType::StringLiteral => TokenType::StringLiteral(self.buffer.clone()),
                ExpectedType::Operator => TokenType::Operator(self.buffer.clone())
            }
        });

        self.buffer.clear();
        self.expected_type = ExpectedType::Identifier;
    }
}

pub fn tokenize<'a>(input : String, state : &'a State) -> Result<Vec<Token>, ParsingError> {
    let mut parser = Parser {
        in_quotes: false,
        tokens: vec![],
        buffer: String::new(),
        position: ParserPosition { line: 0, column: 0 },
        expected_type: ExpectedType::Identifier,
        state: state,
    };

    for char in input.chars() {
        parser.increment_col();

        match char {
            char if char.is_ascii_alphabetic() => {
                parser.buffer.push(char)
            }

            '0' ..= '9' => {

                if parser.buffer.is_empty()
                || parser.buffer.len() == 1 && parser.buffer.starts_with("-") {    
                    parser.expected_type = ExpectedType::Integer;
                }
 
                parser.buffer.push(char);
            }

            ' ' => {
                if parser.in_quotes {
                    parser.buffer.push(' ')
                } else if parser.buffer.len() > 0 {
                    parser.send_buffer();
                }
            }

            '\r' => {
                // Ignore
            }

            '\n' => {
                parser.send_buffer();
                parser.tokens.push(Token{
                    position: parser.position,
                    token_type: TokenType::NewLine,
                });
            }

            '\'' => {
                if parser.in_quotes {
                    parser.send_buffer();
                    parser.in_quotes = false;
                } else {
                    // quotes should not start in the middle of words
                    if parser.buffer.len() > 0 {
                        return Err(ParsingError { 
                            position: parser.position,
                            err_type: ErrorType::MisplacedQuote 
                        });
                    }

                    parser.in_quotes = true;
                    parser.expected_type = ExpectedType::StringLiteral;
                }
            }

            '(' => {
                parser.send_buffer();
                parser.tokens.push(Token{
                    position: parser.position,
                    token_type: TokenType::OpenParentheses
                })
            },

            ')' => {
                parser.send_buffer();
                parser.tokens.push(Token{
                    position: parser.position,
                    token_type: TokenType::CloseParentheses
                })
            },

            _ => {
                parser.buffer.push(char);
            }
        }
    };

    // Cleanup
    if parser.in_quotes {
        return Err(ParsingError {
            position: parser.position,
            err_type: ErrorType::MissingClosingQuote
        })
    }

    parser.send_buffer();

    Ok(parser.tokens)
}