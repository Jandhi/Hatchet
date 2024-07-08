use crate::position::Position;

use super::lexeme::{Identifier, Lexeme, LexemeType, Literal, Separator};

struct Lexer<'a> {
    position : Position,
    counter : usize,
    input : &'a str,
    buffer : String,
    in_quotes : bool,
    lexemes : Vec<Lexeme>,
}

pub fn lex(input : &str) -> Vec<Lexeme> {
    Lexer::new(input).lex()
}

impl<'a> Lexer<'a> {
    pub fn new(input : &'a str) -> Lexer<'a> {
        Lexer { 
            position: Position::new(), 
            counter: 0, 
            input, 
            buffer: "".to_string(), 
            in_quotes: false, 
            lexemes: vec![] 
        }
    }

    pub fn lex(mut self) -> Vec<Lexeme> {

        while let Some(next_symbol) = self.get_next_symbol() {
            match next_symbol {
                '(' if !self.in_quotes => {
                    self.push_buffer();
                    self.push_lexeme(Separator::LeftParen)
                }
                ')' if !self.in_quotes => {
                    self.push_buffer();
                    self.push_lexeme(Separator::RightParen)
                }
                '\'' => {
                    if self.in_quotes {
                        self.push_buffer();
                        self.in_quotes = false;
                    } else {
                        self.push_buffer();
                        self.in_quotes = true;
                    }
                }
                _ => {
                    self.buffer.push(next_symbol)
                }
            }
        }

        self.push_buffer();

        self.lexemes
    }

    pub fn get_next_symbol(&mut self) -> Option<char> {
        if self.counter >= self.input.len() {
            None
        } else {
            self.counter += 1;
            Some(self.input.chars().nth(self.counter - 1).expect("Counter has exceeded the string boundary for some reason"))
        }
    }

    pub fn push_buffer(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        if self.in_quotes {
            self.push_lexeme(Literal::String(self.buffer.clone()));
            self.buffer = "".to_string();
            return;
        }

        self.push_lexeme(Identifier{name : self.buffer.clone()});
        self.buffer = "".to_string();
    }

    pub fn push_lexeme(&mut self, lex_type : impl Into<LexemeType>) {
        self.lexemes.push(Lexeme { position: self.position, lex_type: lex_type.into() })
    }
}