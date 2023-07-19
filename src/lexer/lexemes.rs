use std::{rc::Rc, ops::Deref};

use super::literal::Literal;

pub type Text = Rc<str>;

#[derive(Debug)]
pub enum Lexeme {
    Literal(Literal),
    Identifier(Text),
    Operator(Text)    
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
    "+", "-", "<", ">", "=", ":=", ":", "*", "/", "|", "&", "|>"
];

impl Lexer {
    fn get_text(&self) -> Text {
        Rc::from(self.buffer.as_str())
    }

    fn to_lexeme(&self, text : Text) -> Lexeme {


        for operator in OPERATORS {
            if operator.deref().eq(text.as_ref()) {
                return Lexeme::Operator(text)
            }
        }

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

    pub fn Push(&mut self) {
        if self.buffer.is_empty() {
            return;
        }  

        let text = self.get_text();
        self.lexemes.push(self.to_lexeme(text));
        self.buffer.clear();
    }

    pub fn Consume(&mut self, char : char) {
        match char {
            '"' => {
                if self.in_quotes {
                    self.Push();
                    self.in_quotes = false;
                } else {
                    self.Push();
                    self.in_quotes = true;
                }
            }

            ' ' | '\n' => {
                self.Push()
            }

            letter => self.buffer.push(letter)
        };

        
    }
}