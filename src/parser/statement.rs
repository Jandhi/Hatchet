use std::rc::Rc;

use crate::my_types::Text;

use super::{expression::Expression, program::{CodeWriter}, assignment::Assignee, context::WriterContext};

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Assignment(Assignee, Expression),
}

impl CodeWriter for Statement {
    fn write(&self, buffer : &mut String, program : &super::program::Program, context : &WriterContext) {
        match self {
            Statement::Expression(expr) => expr.write(buffer, program, context),
            Statement::Assignment(assignee, expr) => {
                match assignee {
                    Assignee::Single(name) => todo!(),
                }
            },
        }
    }
}

