use std::vec;

use crate::{parsing::{token::{Token, TokenType::{Identifier, StringLiteral, OpenParentheses, Operator, IntegerLiteral}}, tokenizer::ParserPosition}, expression::{Expression, ExpressionType}, value::Value};

use super::translation_error::TranslationError;

pub fn translate(tokens : &mut Vec<Token>) -> Result<Expression, TranslationError> {
    let mut expressions = vec![];

    while tokens.len() > 0 {
        let result = translate_sequence(tokens);

        match result {
            Ok(expr) => expressions.push(expr),
            Err(err) => return Err(err),
        }
    }

    Ok(Expression{
        position: ParserPosition { line: 0, column: 0 },
        expr_type: ExpressionType::Procedure(expressions)
    })
}

pub fn translate_sequence(tokens : &mut Vec<Token>) -> Result<Expression, TranslationError> {
    let mut expressions : Vec<Expression> = vec![];

    loop {
        let peek = tokens.remove(0);

        if let Operator(name) = peek.token_type {
            if expressions.len() == 0 { // If leading with an operator, treat as normal reference
                expressions.push(Expression { 
                    position: peek.position, 
                    expr_type: ExpressionType::Reference(name) 
                })
            } else { // Else pop last expression and get next, formulate a function call
                match next_expr(tokens) {
                    Ok(b) => {
                        let a = expressions.pop().unwrap();
                        let callee = Box::from(Expression {
                            position : peek.position,
                            expr_type : ExpressionType::Reference(name)
                        });

                        expressions.push(Expression { 
                            position: a.position, 
                            expr_type: ExpressionType::FunctionCall(callee, vec![a, b]) })
                    },
                    Err(err) => return Err(err)
                }
            }

        } else {
            tokens.insert(0, peek);

            match next_expr(tokens) {
                Ok(expr) => {
                    expressions.push(expr);
                },
                Err(err) => return Err(err)
            }
        }

        if tokens.len() == 0 {
            break;
        }
    }

    if expressions.len() == 1 {
        return Ok(expressions.pop().unwrap());
    } else {
        let callee = expressions.remove(0);

        return Ok(Expression{
            position: expressions[0].position,
            expr_type: ExpressionType::FunctionCall(Box::from(callee), expressions)
        });
    }
}

fn next_expr(tokens : &mut Vec<Token>) -> Result<Expression, TranslationError> {
    let token = tokens.remove(0);

    match token.token_type {
        Identifier(name) => {
            return Ok(Expression{
                position: token.position,
                expr_type: ExpressionType::Reference(name),
            })
        },

        Operator(name) => {
            return Ok(Expression{
                position: token.position,
                expr_type: ExpressionType::Reference(name),
            })
        },

        StringLiteral(value) => {
            return Ok(Expression{
                position: token.position,
                expr_type: ExpressionType::Value(Value::String(value)),
            })
        },

        IntegerLiteral(value) => {
            return Ok(Expression { 
                position: token.position, 
                expr_type: ExpressionType::Value(Value::Integer(value))  
            })
        }

        OpenParentheses => {
            return translate_parentheses(token.position, tokens);
        }

        _ => todo!("Can't process {:?}", token)
    }
}

fn translate_parentheses(pos : ParserPosition, tokens : &mut Vec<Token>) -> Result<Expression, TranslationError> {
    let mut inner_tokens : Vec<Token> = vec![];

    loop {
        let token = tokens.remove(0);

        match &token.token_type {
            crate::parsing::token::TokenType::CloseParentheses => return translate_sequence(&mut inner_tokens),
            _ => inner_tokens.push(token),
        }

        if tokens.len() == 0 {
            return Err(TranslationError {
                position: pos,
                err_type: super::translation_error::ErrorType::MissingClosingParentheses,
            });
        }
    }
}