use super::{control_block::ControlBlock, node::{Node, NodeDisplay}, expression::Expression, assignment::Assignment};

#[derive(Debug)]
pub enum Statement {
    ControlBlock(Box<Node<ControlBlock>>),
    Assignment(Box<Node<Assignment>>),
    Block(Vec<Node<Statement>>),
}

impl NodeDisplay for Statement {
    fn node_display(&self, depth : usize) -> String {
        match self {
            Statement::ControlBlock(control_block) => control_block.node_display(depth),
            Statement::Assignment(assign) => assign.node_display(depth),
            Statement::Block(statements) => {
                statements.iter()
                    .map(|statement| statement.node_display(depth))
                    .fold(String::new(), |acc, element| acc + &element + "\n")
                    .trim_end()
                    .to_string()
            },
        }
    }
}