use crate::lexer::{token::{TokenType, Token}, literal::Literal};
use crate::types::hatchet_type::TypeData;

use super::{parser::Parser, node::{Node, NodeDisplay}, error::{ParseError, ParseErrorType}};

#[derive(Debug, Clone)]
pub enum Constant {
    Int(i32),
    String(String),
}

impl NodeDisplay for Constant {
    fn node_display(&self, depth : usize) -> String {
        format!("{}{}", Self::spacing(depth), match self {
            Constant::Int(value) => format!("INT {value}"),
            Constant::String(value) => format!("STR \"{value}\""),
        })
    }
}

impl Parser<Constant, ()> for Constant {
    fn parse(tokens : &mut Vec<&Token>, _context : ()) -> Result<Node<Constant>, ParseError> {
        let token = tokens.remove(0);

        match &token.token_type {
            TokenType::Literal(literal) => match literal {
                Literal::String(value) => Ok(Node {
                    content: Constant::String(value.clone()),
                    position: token.position,
                    type_data: TypeData::unknown(),
                }),
                Literal::Number(value) => {
                    let parsed_value = value.parse::<i32>();
                    
                    match parsed_value {
                        Ok(int_value) => Ok(Node {
                            content: Constant::Int(int_value),
                            position: token.position,
                            type_data: TypeData::unknown(),
                        }),
                        Err(parse_error) => Err(ParseError { 
                            position: token.position, 
                            description: format!("Number format error: {}", parse_error), 
                            error: ParseErrorType::NumberFormat,
                        }),
                    }
                },
                _ => todo!("Literal type {:?} is unsupported.", literal),
            },
            _ => Err(ParseError { 
                position: token.position, 
                description: format!("Expected constant! Received {:?}", token), 
                error: ParseErrorType::UnexpectedToken,
            }),
        }
    }
}