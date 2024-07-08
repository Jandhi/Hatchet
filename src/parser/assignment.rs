use super::{expression::Expression, node::{Node, NodeDisplay}};

#[derive(Debug)]
pub struct Assignment {
    pub identifier : Node<String>,
    pub expr : Node<Expression>,
}

impl NodeDisplay for Assignment {
    fn node_display(&self, depth : usize) -> String {
        format!("ASSIGN {}\n{}", self.identifier.content, self.expr.node_display(depth + 1))
    }
}

