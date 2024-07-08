use std::fmt::Debug;

use crate::lexer::{token::{Token, TokenType}, separator::Separator};

use super::{error::ParseError, node::{Node, NodeDisplay}};
use crate::parser::statement::Statement;

pub trait Parser<TNode : Debug + NodeDisplay, TContext> {
    fn parse(tokens : &mut Vec<&Token>, context : TContext) -> Result<Node<TNode>, ParseError>;
}

