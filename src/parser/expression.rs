use crate::{literal::Literal, types::{hatchet_type::{HatchetType}}, my_types::Text};

use super::{function::call::Call, program::{CodeWriter}, context::Context, variables::find_variable};


#[derive(Debug)]
pub struct Expression {
    pub expr_type : ExpressionType,
    pub my_type : HatchetType
}

#[derive(Debug)]
pub enum ExpressionType {
    Literal(Literal),
    FunctionCall(Call),
    VariableRead(Text),
}

impl ExpressionType {
    pub fn as_expr(self) -> Expression {
        Expression { expr_type: self, my_type: HatchetType::Unknown }
    }
}

impl CodeWriter for Expression {
    fn write(&self, buffer : &mut String,  context : &mut Context) {
        match &self.expr_type {
            ExpressionType::Literal(literal) => literal.write(buffer, context),
            ExpressionType::FunctionCall(call) => call.write(buffer, context),
            ExpressionType::VariableRead(name) => {
                let variable = find_variable(name, context);
                variable.write(buffer, &mut context.clone());
            },
        };
    }
}

