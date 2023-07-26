use std::ops::Deref;

use crate::{parser::{program::Program, context::Context, expression::{Expression, ExpressionType}, variables::{find_variable, Variable}, function::call::Call, statement::Statement, assignment::Assignee}, literal::{Literal, self}};

use super::{hatchet_type::HatchetType, primitive_type::{INT_TYPE, STRING_TYPE}};

pub trait HasType {
    fn set_type(&mut self, my_type : HatchetType);
    fn get_type(&self) -> HatchetType;
}

pub trait TypeChecker {
    fn check_type(&mut self, context : &mut Context);
}

impl HasType for Literal {
    fn set_type(&mut self, my_type : crate::types::hatchet_type::HatchetType) {
        // nothing needed
    }

    fn get_type(&self) -> crate::types::hatchet_type::HatchetType {
        match &self {
            Literal::String(_) => STRING_TYPE,
            Literal::Int32(_) => INT_TYPE,
            Literal::UInt32(_) => todo!(),
        }
    }
}

impl TypeChecker for Expression {
    fn check_type(&mut self, context : &mut Context) {
        let my_type = match &mut self.expr_type {
            ExpressionType::Literal(literal) => literal.get_type(),
            ExpressionType::FunctionCall(call) => {
                call.check_type(context);
                call.get_type()
            },
            ExpressionType::VariableRead(name) => {
                let variable = find_variable(name, context);
                variable.htype.clone()
            },
        };

        self.set_type(my_type);
    }
}

impl HasType for Expression {
    

    fn set_type(&mut self, my_type : HatchetType) {
        self.my_type = my_type
    }

    fn get_type(&self) -> HatchetType {
        self.my_type.clone()
    }
}

impl TypeChecker for Call {
    fn check_type(&mut self, context : &mut Context) {
        for expr in &mut self.args {
            expr.check_type(context);
        }

        for func in &context.functions {
            if self.is_match(func) {
                self.set_type(func.return_type.clone());
                return;
            }
        }

        panic!("No matching function found for {}", self.signature())
    }
}

impl HasType for Call {
    fn set_type(&mut self, my_type : HatchetType) {
        self.my_type = my_type
    }

    fn get_type(&self) -> HatchetType {
        self.my_type.clone()
    }
}

impl TypeChecker for Statement {
    fn check_type(&mut self, context : &mut Context) {
        match self {
            Statement::Expression(expr) => expr.check_type(context),
            Statement::Assignment(assignee, expr) => {
                match assignee {
                    Assignee::Single(name) => {
                        expr.check_type(context);

                        if let Some(var) = context.variables.iter().filter(|var| &var.name == name).last() {
                            assert!(expr.get_type().is_type(&var.htype), "Assigning {} to {}", expr.get_type().get_name(), var.htype.get_name());
                        } else {
                            context.variables.push(Variable{
                                name: name.clone(),
                                htype: expr.get_type(),
                            })
                        }                        
                    },
                }
            }
        }
    }
}