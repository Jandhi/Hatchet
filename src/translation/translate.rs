use std::vec;

use crate::{parsing::token::{Token, TokenType::{Identifier, StringLiteral}}, expression::Expression, function::Procedure, value::Value};

use super::translation_error::TranslationError;

pub fn translate(tokens : &mut Vec<Token>) -> Result<Expression, TranslationError> {
    let mut expressions = vec![];

    while tokens.len() > 0 {
        let result = translate_line(tokens);

        match result {
            Ok(expr) => expressions.push(expr),
            Err(err) => return Err(err),
        }
    }

    Ok(Expression::Procedure(expressions))
}

pub fn translate_line(tokens : &mut Vec<Token>) -> Result<Expression, TranslationError> {
    let token = tokens.remove(0);

    match token.token_type {
        Identifier(name) => {
            if tokens.len() == 0 {
                // The expression is an identifier
                Ok(Expression::Reference(name))
            } else {
                translate_function_call(name, tokens)
            }
        }
        _ => todo!("Need to process that type")
    }
}

pub fn translate_function_call(name : String, tokens : &mut Vec<Token>) -> Result<Expression, TranslationError> {
    let mut args = vec![];
    
    loop {
        if tokens.len() == 0 {
            break;
        }

        let token = tokens.remove(0);

        match token.token_type {
            Identifier(name) => {
                args.push(Expression::Reference(name))
            },

            StringLiteral(value) => {
                args.push(Expression::Value(Value::String(value)))
            }

            _ => panic!("This should not happen!")
        };
    }

    Ok(Expression::FunctionCall(Box::from(Expression::Reference(name)), args))
}