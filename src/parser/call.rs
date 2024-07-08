use crate::{lexer::{token::{Token, TokenType}, separator::Separator}, types::hatchet_type::TypeData};

use super::{expression::Expression, parser::Parser, error::{ParseError, ParseErrorType}, node::{Node, NodeDisplay}};

#[derive(Debug)]
pub struct FunctionCall {
    pub function : Box<Node<Expression>>,
    pub arguments : Vec<Node<Expression>>,
}

impl NodeDisplay for FunctionCall {
    fn node_display(&self, depth : usize) -> String {
        let tabs = Self::spacing(depth);

        let mut string = format!("{}{}", tabs, "FUNCTION CALL\n");
        
        for arg in self.arguments.iter() {
            string += &arg.node_display(depth + 1);
            string +=  "\n";
        }

        string += &self.function.node_display(depth + 1);
        
        string
    }
}

impl Parser<FunctionCall, ()> for FunctionCall {
    fn parse(tokens : &mut Vec<&Token>, context : ()) -> Result<Node<FunctionCall>, ParseError> {
        let position = tokens[0].position;
        let first_len = find_first_expr_len(tokens)?;
        let mut callee_tokens = remove_first_n(tokens, first_len);
        let callee = Expression::parse(&mut callee_tokens, ())?;

        let mut args : Vec<Node<Expression>> = vec![];

        while tokens.len() > 0 {
            let first_len = find_first_expr_len(tokens)?;
            let mut arg_tokens = remove_first_n(tokens, first_len);
            let arg = Expression::parse(&mut arg_tokens, ())?;
            args.push(arg);
        }

        Ok(Node {
            content: FunctionCall {
                function: Box::from(callee),
                arguments: args,
            },
            position: position,
            type_data: TypeData::unknown(),
        })
    }
}

fn remove_first_n<'a>(tokens : &mut Vec<&'a Token>, amount : usize) -> Vec<&'a Token> {
    let mut new_vec : Vec<&Token> = vec![];

    for _ in 0..amount {
        new_vec.push(tokens.remove(0));
    }

    new_vec
}

fn find_first_expr_len(tokens : &Vec<&Token>) -> Result<usize, ParseError> {
    let mut index = 0;
    let mut depth = 0;

    while index < tokens.len() {
        match &tokens[index].token_type {
            TokenType::Identifier(iden) => {
                if depth == 0 {
                    return Ok(index + 1);
                }
            }
            TokenType::Separator(Separator::LeftParen) => {
                depth += 1;
            }
            TokenType::Separator(Separator::RightParen) => {
                if depth > 0 {
                    depth -= 1;

                    if depth == 0 {
                        return Ok(index + 1)
                    }
                } else {
                    return Err(ParseError { 
                        position: tokens.last().unwrap().position, 
                        description: format!("No matching left parentheses for right parentheses!"), 
                        error: ParseErrorType::MalformedParentheses,
                    });
                }
            }
            _ => {}
        }

        index += 1;
    }

    Err(ParseError { 
        position: tokens.last().unwrap().position, 
        description: format!("Left parentheses is left unclosed!"), 
        error: ParseErrorType::MalformedParentheses
    })
}