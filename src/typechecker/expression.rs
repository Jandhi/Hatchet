use crate::{parser::{expression::Expression, node::Node}, types::hatchet_type};

use super::typechecker::{TypeCheckable, TypeContext};

impl TypeCheckable for Node<Expression> {
    fn type_check(&mut self, ctx : &mut TypeContext) -> &hatchet_type::TypeData {
        match &mut self.content {
            Expression::Constant(constant) => self.type_data = constant.type_check(ctx).clone(),
            Expression::Identifier(iden) => {
                match ctx.get_type(iden) {
                    Some(_) => todo!(),
                    None => {
                        
                    },
                }
            },
            Expression::UnaryExpr(_) => todo!(),
            Expression::BinaryExpr(_) => todo!(),
            Expression::FunctionCall(_) => todo!(),
            Expression::Statement(statement) => todo!(),
        };
        &self.type_data
    }
}