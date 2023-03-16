use std::vec;

use colored::Colorize;

use crate::{parsing::{token::{Token, TokenType::{Identifier, StringLiteral, OpenParentheses, Operator, IntegerLiteral, self}}, tokenizer::ParserPosition}, expression::{Expression, ExpressionType}, value::Value};

use super::translation_error::{TranslationError, ErrorType};

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
        if tokens.len() == 0 {
            break;
        }

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
                            expr_type: ExpressionType::FunctionCall(callee, vec![a, b]) 
                        })
                    },
                    Err(err) => return Err(err)
                }
            }

        } else if let TokenType::NewLine = peek.token_type {
            break; // end read loop
        } else if let TokenType::Pipe = peek.token_type {
            let left = pack_up(&mut expressions);
            let mut right_tokens = vec![];
            let mut return_expr = false; 

            loop {
                if tokens.len() == 0 {
                    return_expr = true;
                    break;
                }

                let next = tokens.remove(0);

                match next.token_type {
                    TokenType::Pipe => {
                        tokens.push(next);
                        break;
                    }
                    
                    TokenType::NewLine => {
                        return_expr = true;
                        break;
                    }

                    _ => {
                        right_tokens.push(next);
                    }
                }
            }

            println!("{} {}", "RTOKENS:".red(), right_tokens.len());
            for token in right_tokens.iter() {
                println!("{:?}", token.token_type);
            }

            match translate_sequence(&mut right_tokens) {
                Ok(right) => {
                    let expr = Expression { 
                        position: peek.position, 
                        expr_type: ExpressionType::Pipe(Box::from(left), Box::from(right)) 
                    };

                    if return_expr {
                        return Ok(expr);
                    } else {
                        expressions.push(expr);
                    }
                },
                Err(_) => todo!(),
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

    return Ok(pack_up(&mut expressions))
}

fn pack_up(expressions : &mut Vec<Expression>) -> Expression {
    if expressions.len() == 1 {
        return expressions.pop().unwrap();
    } else {
        let callee = expressions.remove(0);
        let mut args = vec![];

        while let Some(expr) = expressions.pop()  {
            args.push(expr);
        }

        return Expression{
            position: callee.position,
            expr_type: ExpressionType::FunctionCall(Box::from(callee), args)
        }
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

        TokenType::Boolean(value) => {
            return Ok(Expression {
                position: token.position, 
                expr_type: ExpressionType::Value(Value::Boolean(value)), 
            });
        }

        OpenParentheses => {
            return translate_parentheses(token.position, tokens);
        }

        _ => return Err(TranslationError {
            position: token.position,
            err_type: ErrorType::UnexpectedToken(token.token_type)
        })
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
                err_type: ErrorType::MissingClosingParentheses,
            });
        }
    }
}