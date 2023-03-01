use crate::parsing::token::Token;
use super::{parsing_error::{ParsingError, ErrorType}, token::TokenType::{StringLiteral, Identifier}};

#[derive(Debug, Clone, Copy)]
pub struct ParserPosition {
    pub line : u16,
    pub column : u16
}


pub fn tokenize(input : String) -> Result<Vec<Token>, ParsingError> {
    let mut in_quotes = false;
    let mut tokens = vec![];
    let mut buffer = String::from("");
    let mut position = ParserPosition{ line: 0, column: 0 };

    for char in input.chars() {
        position.column += 1; // increment position

        match char {
            char if char.is_ascii_alphabetic() => {
                buffer.push(char)
            }

            ' ' => {
                if in_quotes {
                    buffer.push(' ')
                } else if buffer.len() > 0 {
                    tokens.push(Token { position, token_type: Identifier(buffer.clone()) });
                    buffer.clear();
                }
            }

            '\'' => {
                if in_quotes {
                    // ends string
                    tokens.push(Token { position, token_type: StringLiteral(buffer.clone()) });
                    buffer.clear();
                    in_quotes = false;
                } else {
                    // quotes should not start in the middle of words
                    if buffer.len() > 0 {
                        return Err(ParsingError { 
                            position,
                            err_type: ErrorType::MisplacedQuote 
                        });
                    }

                    in_quotes = true;
                }
            }

            _ => {
                return  Err(ParsingError {
                    position,
                    err_type: ErrorType::UnexpectedCharacter(char)
                });
            }
        }
    };

    // Cleanup
    if in_quotes {
        return Err(ParsingError {
            position,
            err_type: ErrorType::MissingClosingQuote
        })
    }

    if buffer.len() > 0 {
        tokens.push(Token { position, token_type: Identifier(buffer.clone()) });
    }

    Ok(tokens)
}