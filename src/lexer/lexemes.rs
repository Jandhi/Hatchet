use std::{rc::Rc, ops::Deref};
use crate::my_types::Text;
use crate::literal::Literal;

#[derive(Debug, Clone)]
pub enum Lexeme {
    Literal(Literal),
    Identifier(Text),
    Operator(Text),
    UnaryOperator(Text),

    Assignment,

    Newline,
    LeftParen,
    RightParen,

    Pipe,
}

pub fn make_lexer() -> Lexer {
    return Lexer { in_quotes : false, buffer: String::from(""), lexemes: vec![] }
}

pub struct Lexer {
    in_quotes : bool,
    buffer : String,
    pub lexemes : Vec<Lexeme>
}

const OPERATORS: &'static [&'static str] = &[
    "+", "-", "<", ">", "=", ":=", ":", "*", "/", "|", "&"
];

impl Lexer {
    fn get_text(&self) -> Text {
        Rc::from(self.buffer.as_str())
    }

    fn to_lexeme(&self, text : Text) -> Lexeme {
        if self.in_quotes {
            return Lexeme::Literal(Literal::String(text));
        }

        if text.deref() == "|>" {
            return Lexeme::Pipe
        }

        if text.deref() == ":=" {
            return Lexeme::Assignment
        }

        for operator in OPERATORS {
            if operator.deref().eq(text.as_ref()) {
                return Lexeme::Operator(text)
            }
        }

        // Is Integer
        if text.chars().all(|c| {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
                _ => false
            }
        }) {
            return Lexeme::Literal(Literal::Int32(text.parse::<i32>().expect("This should have parsed")));
        }

        return Lexeme::Identifier(text);
    }

    pub fn push(&mut self) {
        if self.buffer.is_empty() {
            return;
        }  

        let text = self.get_text();
        self.lexemes.push(self.to_lexeme(text));
        self.buffer.clear();
    }

    pub fn consume(&mut self, char : char) {
        match char {
            '"' => {
                if self.in_quotes {
                    self.push();
                    self.in_quotes = false;
                } else {
                    self.push();
                    self.in_quotes = true;
                }
            }

            ' ' | '\n' if !self.in_quotes => {
                self.push()
            }

            letter => self.buffer.push(letter)
        };

        
    }
}

impl PartialEq for Lexeme {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::Operator(l0), Self::Operator(r0)) => l0 == r0,
            (Self::UnaryOperator(l0), Self::UnaryOperator(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}