use std::rc::Rc;

use crate::lexer::lexeme::Lexeme;

use super::node::Node;

struct Parser<'a> {
    lexemes : &'a Vec<Lexeme>
}

pub fn parse(lexemes : &Vec<Lexeme>) -> Rc<Node> {
    Parser::new(lexemes).parse()
}

impl<'a> Parser<'a> {
    fn new(lexemes : &'a Vec<Lexeme>) -> Parser<'a> {
        Parser { lexemes }
    }

    fn parse(&mut self) -> Rc<Node> {
        Rc::from(Node{
            position: todo!(),
            children: todo!(),
            hatchet_type: todo!(),
        })
    }
}