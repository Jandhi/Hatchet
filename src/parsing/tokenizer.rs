use crate::{parsing::token::Token, value::IntegerVal};
use super::{parsing_error::{ParsingError, ErrorType}, token::TokenType::{StringLiteral, Identifier, self}};

#[derive(Debug, Clone, Copy)]
pub struct ParserPosition {
    pub line : u16,
    pub column : u16
}

pub struct Parser {
    pub in_quotes : bool,
    pub tokens : Vec<Token>,
    pub buffer : String,
    pub expected_type : ExpectedType,
    pub position : ParserPosition,
}

pub enum ExpectedType {
    Identifier,
    Integer,
    StringLiteral,
}

impl Parser {
    fn increment_col(&mut self) {
        self.position.column += 1;
    }

    fn send_buffer(&mut self) {
        if self.buffer.len() == 0 {
            return;
        }

        self.tokens.push(Token { 
            position: self.position.clone(),
            token_type: match self.expected_type {
                ExpectedType::Identifier => TokenType::Identifier(self.buffer.clone()),
                ExpectedType::Integer => TokenType::IntegerLiteral(self.buffer.clone().parse().unwrap()),
                ExpectedType::StringLiteral => TokenType::StringLiteral(self.buffer.clone())
            }
        });

        self.buffer.clear();
        self.expected_type = ExpectedType::Identifier;
    }
}

pub fn tokenize(input : String) -> Result<Vec<Token>, ParsingError> {
    let mut parser = Parser {
        in_quotes: false,
        tokens: vec![],
        buffer: String::new(),
        position: ParserPosition { line: 0, column: 0 },
        expected_type: ExpectedType::Identifier,
    };

    for char in input.chars() {
        parser.increment_col();

        match char {
            char if char.is_ascii_alphabetic() => {
                parser.buffer.push(char)
            }

            '0' ..= '9' => {
                if parser.buffer.is_empty() {
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

            '+' | '-' | '*' | '/' => {
                parser.send_buffer();
                let mut operator_name = String::new();
                operator_name.push(char);

                parser.tokens.push(Token{
                    position: parser.position,
                    token_type: TokenType::Operator(operator_name)
                })
            }

            _ => {
                return  Err(ParsingError {
                    position: parser.position,
                    err_type: ErrorType::UnexpectedCharacter(char)
                });
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