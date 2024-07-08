use crate::position::Position;

use super::{
    error::{LexError, LexErrorType},
    keyword::KeyWord,
    literal::Literal,
    separator::Separator,
    token::{Token, TokenType}, operator::Operator,
};

#[derive(PartialEq, Clone, Copy)]
pub enum BufferState {
    Empty,
    Number,
    Word, // KeyWord or Identifier
    String,
    Symbol, // Catch all for not the above
}

pub struct Lexer {
    buffer_state: BufferState,
    position: Position,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            position: Position {
                line: 0,
                col: 0,
                char: 0,
            },
            buffer_state: BufferState::Empty,
        }
    }

    pub fn process(input: &str) -> Result<Vec<Token>, LexError>{
        let mut tokens = vec![];
        let mut stack = input.to_string();
        let mut buffer = String::new();
        let mut lexer = Lexer::new();

        stack = stack.replace("    ", "\t");

        while stack.len() > 0 {
            let curr = stack.remove(0);
            if let Some(err) = lexer.process_char(curr, &mut buffer, &mut tokens) {
                return Err(err);
            };
        }

        if let Some(err) = lexer.push_buffer(&mut buffer, &mut tokens) {
            return Err(err);
        }

        Ok(tokens)
    }

    fn push_buffer(&mut self, buffer: &mut String, tokens: &mut Vec<Token>) -> Option<LexError> {
        match self.buffer_state {
            BufferState::Empty => (),
            BufferState::Number => {
                tokens.push(Token{
                    token_type: TokenType::Literal(Literal::Number(buffer.to_string())),
                    position: self.position,
                });
            }
            BufferState::Word => {
                if let Some(keyword) = KeyWord::get(&buffer) {
                    tokens.push(Token{
                        token_type: TokenType::KeyWord(keyword),
                        position: self.position,
                    });
                } else {
                    tokens.push(Token{
                        token_type: TokenType::Identifier(buffer.to_string()),
                        position: self.position,
                    });
                }
            }
            BufferState::String => {
                tokens.push(Token{
                    token_type: TokenType::Literal(Literal::String(buffer.to_string())),
                    position: self.position,
                });
            }
            BufferState::Symbol => {
                if let Some(sep) = Separator::get(&buffer) {
                    tokens.push(Token{
                        token_type: TokenType::Separator(sep),
                        position: self.position,
                    });
                }
                else if let Some(op) = Operator::get(&buffer) {
                    tokens.push(Token{
                        token_type: TokenType::Operator(op),
                        position: self.position,
                    });
                } else {
                    return Some(LexError { 
                        error: LexErrorType::UnrecognizedOperator, 
                        description: format!("Unrecognized operator: \"{}\"", buffer), 
                        position: self.position,
                    })
                }
            },
        }

        buffer.clear();
        self.buffer_state = BufferState::Empty;

        None
    }

    fn process_char(
        &mut self,
        curr: char,
        buffer: &mut String,
        tokens: &mut Vec<Token>,
    ) -> Option<LexError> {
        match curr {
            // STRING HANDLING
            '\"' => {
                if self.buffer_state == BufferState::String {
                    tokens.push(Token{
                        token_type: TokenType::Literal(Literal::String(buffer.to_string())),
                        position: self.position,
                    });
                    buffer.clear();
                    self.buffer_state = BufferState::Empty;
                } else {
                    self.push_buffer(buffer, tokens);
                    self.buffer_state = BufferState::String;
                }
            }

            _ if self.buffer_state == BufferState::String => {
                buffer.push(curr);
            }

            // NUMBER HANDLING
            '0'..='9' if self.buffer_state == BufferState::Empty => {
                self.buffer_state = BufferState::Number;
                buffer.push(curr);
            }

            '0'..='9' if self.buffer_state == BufferState::Number => {
                buffer.push(curr);
            }

            '0'..='9' if self.buffer_state == BufferState::Symbol => {
                if let Some(err) = self.push_buffer(buffer, tokens) {
                    return Some(err);
                }
                self.buffer_state = BufferState::Number;
                buffer.push(curr);
            }

            'a'..='z' | 'A'..='Z' if self.buffer_state == BufferState::Number => {
                return Some(LexError {
                    error: LexErrorType::UnexpectedSymbol,
                    description: format!("Number cannot end with {}", curr),
                    position: self.position,
                });
            }

            // IDENTIFIER
            'a'..='z' | 'A'..='Z' => {
                if self.buffer_state == BufferState::Word {
                    buffer.push(curr);
                } else {
                    if let Some(err) = self.push_buffer(buffer, tokens) {
                        return Some(err);
                    }
                    self.buffer_state = BufferState::Word;
                    buffer.push(curr);
                }
            }

            '0'..='9' => {
                buffer.push(curr);
            }

            // WHITESPACE
            ' ' => {
                if let Some(err) = self.push_buffer(buffer, tokens) {
                    return Some(err);
                }
            }

            // SYMBOLS
            _ => {
                // Separators take priority
                if let Some(separator) = Separator::get(&curr.to_string()) {
                    if let Some(err) = self.push_buffer(buffer, tokens) {
                        return Some(err);
                    }

                    tokens.push(Token{
                        token_type: TokenType::Separator(separator),
                        position: self.position,
                    });

                // Otherwise, if its a symbol continue it
                // Otherwise start a new symbol
                } else {
                    match self.buffer_state {
                        BufferState::Number | BufferState::Word => {
                            if let Some(err) = self.push_buffer(buffer, tokens) {
                                return Some(err);
                            }
                        }
                        _ => ()
                    };

                    self.buffer_state = BufferState::Symbol;
                    buffer.push(curr);
                }
            },
        }

        // Advance position of lexer
        match curr {
            '\n' => {
                self.position.line += 1;
                self.position.col = 0;
            }
            '\t' => {
                self.position.line += 4;
            }
            _ => {
                self.position.col += 1;
            }
        }
        self.position.char += 1;

        None
    }
}
