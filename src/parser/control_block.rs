use super::{node::{Node, NodeDisplay}, expression::Expression};
use crate::parser::statement::Statement;

#[derive(Debug)]
pub enum ControlBlock {
    If(If),
}

impl NodeDisplay for ControlBlock {
    fn node_display(&self, depth : usize) -> String {
        let tabs = Self::spacing(depth);

        match self {
            ControlBlock::If(if_block) => {
                let mut string = format!("{tabs}IF");
                string += &if_block.condition.node_display(depth + 1);
                string +=  "\n";
                string += &format!("{tabs}THEN");
                string +=  "\n";
                string += &if_block.then.node_display(depth + 1);
                string
            },
        }
    }
}

#[derive(Debug)]
pub struct If {
    pub condition : Node<Expression>,
    pub then : Box<Node<Statement>>,
}

