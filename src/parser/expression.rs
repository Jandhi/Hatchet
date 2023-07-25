

use crate::{literal::Literal, types::hatchet_type::{HasType, HatchetType}};

use super::{function::call::Call, program::{CodeWriter, Program}, context::WriterContext};


#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    FunctionCall(Call),
}

impl CodeWriter for Expression {
    fn write(&self, buffer : &mut String, program : &Program, context : &WriterContext) {
        match &self {
            Expression::Literal(literal) => literal.write(buffer, program, context),
            Expression::FunctionCall(call) => call.write(buffer, program, context),
        };
    }
}

impl HasType for Expression {
    fn get_type(&self, program : &Program) -> HatchetType {
        match &self {
            Expression::Literal(literal) => literal.get_type(program),
            Expression::FunctionCall(call) => {
                return call.get_func(program).return_type.clone()
            },
        }
    }
}