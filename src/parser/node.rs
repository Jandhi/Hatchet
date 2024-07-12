use std::rc::Rc;

use crate::{lexer::lexeme::Lexeme, position::Position, typechecker::hatchet_type::HatchetType};

pub struct Node {
    pub position : Position,
    pub children : Vec<Rc<Node>>,
    pub node_type : NodeType,
    pub hatchet_type : HatchetType
}

pub enum NodeType {
    Root,
    Lexeme(Lexeme),
    
}