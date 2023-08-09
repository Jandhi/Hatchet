use std::{ops::Deref};

use crate::types::{type_checker::HasType, hatchet_type::{HatchetType}};

use super::{expression::Expression, program::{CodeWriter}, assignment::Assignee, context::{Context}, variables::{Variable, find_variable}};

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Assignment(Assignee, Expression),
}

impl CodeWriter for Statement {
    fn write(&self, buffer : &mut String, context : &mut Context) {
        match self {
            Statement::Expression(expr) => expr.write(buffer, context),
            Statement::Assignment(assignee, expr) => {
                write_assignment(assignee, expr, buffer, context);
            },
        }
    }
}

fn write_assignment(assignee : &Assignee, expr : &Expression, buffer : &mut String, context : &mut Context) {
    match assignee {
        Assignee::Single(name) => {
            if context.variables.iter().any(|var| var.name.deref() == name.deref()) {
                let var = find_variable(name, context);
                var.write(buffer, &mut context.clone());
                buffer.push_str(" = ");
                expr.write(buffer, context);
            } else {
                let variable = Variable { 
                    name: name.clone(), 
                    htype: expr.get_type(),
                    is_constant: true,
                };

                if let HatchetType::Function(_func) = variable.htype.clone() {
                    // Write function pointer!
                    

                    // I should do this through typedefs
                    todo!();
                } else {
                    variable.htype.write(buffer, context);
                    buffer.push(' ');
                    variable.write(buffer, context);
                }
                
                buffer.push_str(" = ");
                expr.write(buffer, context);

                // Add variable to local context
                context.variables.push(variable);
            }
        },
    }
}