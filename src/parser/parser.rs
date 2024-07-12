use std::rc::Rc;

use crate::{lexer::lexeme::Lexeme, position::Position, typechecker::hatchet_type::HatchetType};

use super::node::{Node, NodeType};

struct Parser<'a> {
    lexemes : &'a Vec<Lexeme>,
    counter : usize,
    stack : Vec<Rc<Node>>,
}

pub fn parse(lexemes : &Vec<Lexeme>) -> Rc<Node> {
    Parser::new(lexemes).parse()
}

impl<'a> Parser<'a> {
    fn new(lexemes : &'a Vec<Lexeme>) -> Parser<'a> {
        Parser { 
            lexemes, 
            counter: 0, 
            stack: vec![],
        }
    }

    fn parse(&mut self) -> Rc<Node> {
        let mut root = Rc::from(Node{
            position: Position::new(),
            children: vec![],
            node_type: NodeType::Root,
            hatchet_type: HatchetType::Unknown,
        });

        self.stack.push(root.clone());

        while let Some(lex) = self.get_next_lexeme() {
            self.stack.push(Rc::from(Node{
                position: lex.position.clone(),
                children: vec![],
                node_type: NodeType::Lexeme(lex),
                hatchet_type: HatchetType::Unknown,
            }));

            while self.reduce() {}
        }

        root
    }

    fn get_next_lexeme(&mut self) -> Option<Lexeme> {
        if self.counter < self.lexemes.len() {
            self.counter += 1;
            return Some(self.lexemes[self.counter - 1].clone());
        }
        
        None
    }

    fn reduce(&mut self) -> bool {
        false
    }
}