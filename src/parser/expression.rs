use crate::{lexer::{token::{self, TokenType, Token, print_tokens}, operator::Operator as OperatorToken, separator::Separator}, types::hatchet_type::TypeData};

use super::{constant::Constant, operator::Operator, call::FunctionCall, node::{Node, NodeDisplay}, parser::Parser, error::{ParseError, ParseErrorType}, statement::Statement};

#[derive(Debug)]
pub enum Expression {
    Constant(Node<Constant>),
    Identifier(String),
    UnaryExpr(Node<UnaryExpr>),
    BinaryExpr(Node<BinaryExpr>),
    FunctionCall(Node<FunctionCall>),
    Statement(Node<Statement>,)
}

impl NodeDisplay for Expression {
    fn node_display(&self, depth : usize) -> String {
        match self {
            Expression::Constant(constant) => constant.node_display(depth),
            Expression::Identifier(name) => format!("{}IDENTIFIER: {}", Self::spacing(depth), name),
            Expression::UnaryExpr(expr) => expr.node_display(depth),
            Expression::BinaryExpr(expr) => expr.node_display(depth),
            Expression::FunctionCall(call) => call.node_display(depth),
            Expression::Statement(statement) => statement.node_display(depth),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator : Operator,
    pub operator_first : bool,
    pub expr : Box<Node<Expression>>,
}

impl NodeDisplay for UnaryExpr {
    fn node_display(&self, depth : usize) -> String {
        let tabs = Self::spacing(depth);

        let mut string = format!("{tabs}UNARY EXPR\n");
        
        if self.operator_first {
            string += &format!("{}{:?}", Self::spacing(depth + 1), self.operator);
            string +=  "\n";
            string += &self.expr.node_display(depth + 1);
        } else {
            string += &self.expr.node_display(depth + 1);
            string +=  "\n";
            string += &format!("{}{:?}", Self::spacing(depth + 1), self.operator);
        }

        string
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub operator : Operator,
    pub left : Box<Node<Expression>>,
    pub right : Box<Node<Expression>>,
}

impl NodeDisplay for BinaryExpr {
    fn node_display(&self, depth : usize) -> String {
        let tabs = Self::spacing(depth);

        let mut string = format!("{tabs}BINARY EXPR\n");
        string += &self.left.node_display(depth + 1);
        string +=  "\n";
        string += &format!("{}{:?}", Self::spacing(depth + 1), self.operator);
        string +=  "\n";
        string += &self.right.node_display(depth + 1);

        string
    }
}

impl Parser<Expression, ()> for Expression {
    // Assumes the whole line is the expression
    fn parse(tokens : &mut Vec<&Token>, _context : ()) -> Result<Node<Expression>, ParseError> {
        let position = tokens[0].position;

        // Remove surrounding brackets
        if in_parens(tokens)?
        {
            tokens.remove(0); 
            tokens.remove(tokens.len() - 1);

            return Expression::parse(tokens, ());
        }

        // Parse single token expressions
        if tokens.len() == 1 {
            match &tokens[0].token_type {
                TokenType::Identifier(iden) => {
                    return Ok(Node {
                        content: Expression::Identifier(iden.clone()),
                        position: position,
                        type_data: TypeData::unknown(),
                    })
                },
                TokenType::Literal(_) => {
                    return Ok(Node { 
                        content: Expression::Constant(Constant::parse(tokens, ())?), 
                        position: position, 
                        type_data: TypeData::unknown(),
                    })
                },
                _ => {
                    return Err(ParseError { 
                        position: position, 
                        description: format!("Invalid expression format!"), 
                        error: ParseErrorType::InvalidExpression 
                    })
                }
                
            }
        }

        match find_lowest_precedence_operator(tokens) {
            Some(op_index) => {
                if op_index == 0 {
                    return Ok(Node { 
                        content: Expression::UnaryExpr(UnaryExpr::parse(tokens, ())?), 
                        position: position, 
                        type_data: TypeData::unknown(),
                    })
                } else {
                    return Ok(Node {
                        content: Expression::BinaryExpr(
                            BinaryExpr::parse(tokens, BinaryExprParseContext { operator_index: op_index })?
                        ),
                        position: position,
                        type_data: TypeData::unknown(),
                    })
                }
            },
            None => {
                return Ok(Node {
                    content: Expression::FunctionCall(
                        FunctionCall::parse(tokens, ())?
                    ),
                    position: position,
                    type_data: TypeData::unknown(),
                })
            },
        }
    }
}

fn in_parens(tokens : &Vec<&Token>) -> Result<bool, ParseError> {
    let mut depth = 0;
    let mut starts_with_paren = false;
    let mut start_paren_unclosed = true;
    let mut ends_with_paren = false;

    for (i, token) in tokens.iter().enumerate() {
        if let TokenType::Separator(Separator::LeftParen) = token.token_type {
            if i == 0 {
                starts_with_paren = true;
            }

            depth += 1;
        } else if let TokenType::Separator(Separator::RightParen) = token.token_type {
            if depth == 0 {
                return Err(ParseError {
                    position: token.position,
                    description: format!("No matching left parentheses for right parentheses!"), 
                    error: ParseErrorType::MalformedParentheses,
                });
            } else {
                depth -= 1;

                if i == tokens.len() - 1 {
                    ends_with_paren = true;
                } else if depth == 0 {
                    // Start paren has been closed
                    start_paren_unclosed = false; 
                }
            }
        }
    }

    if depth > 0 {
        return Err(ParseError {
            position: tokens.last().unwrap().position,
            description: format!("Left parentheses is left unclosed!"),
            error: ParseErrorType::MalformedParentheses,
        });
    }

    

    Ok(starts_with_paren && ends_with_paren && start_paren_unclosed)
}

fn find_lowest_precedence_operator(tokens : &Vec<&Token>) -> Option<usize> {
    let mut highest : Option<usize> = None;
    let mut depth = 0;

    for i in 0..tokens.len() {
        if let TokenType::Separator(Separator::LeftParen) = tokens[i].token_type {
            depth += 1;
        }
        else if let TokenType::Separator(Separator::RightParen) = tokens[i].token_type {
            depth -= 1;
        }
        else if let TokenType::Operator(op_token) = tokens[i].token_type {
            if depth > 0 {
                continue;
            }

            match highest {
                Some(highest_index) => {
                    if let TokenType::Operator(highest_op) = tokens[highest_index].token_type {
                        if !op_token.is_higher_precedence(highest_op) {
                            highest = Some(i);
                        }
                    } else {
                        return None;
                    }
                },
                None => highest = Some(i),
            }
        }
    }

    highest
}



impl Parser<UnaryExpr, ()> for UnaryExpr {
    fn parse(tokens : &mut Vec<&Token>, _context : ()) -> Result<Node<UnaryExpr>, ParseError> {
        let token = tokens.remove(0);

        match &token.token_type {
            TokenType::Operator(op_token) => {
                let expression = Expression::parse(tokens, ())?;

                let operator = match op_token {
                    _ => {
                        Err(ParseError { 
                            position: token.position, 
                            description: format!("Operator {:?} is not supported for leading unary operations!", op_token), 
                            error: ParseErrorType::UnsupportedOperator,
                        })
                    }   
                }?;

                Ok(Node::<UnaryExpr> {
                    content: UnaryExpr { operator, expr: Box::from(expression), operator_first: true },
                    position: token.position,
                    type_data: TypeData::unknown(),
                })
            }
            _ => {
                tokens.insert(0, token);
                let token = tokens.remove(tokens.len() - 1);
                if let TokenType::Operator(op_token) = token.token_type {
                    let expression = Expression::parse(tokens, ())?;

                    let operator = match op_token {
                        _ => {
                            Err(ParseError { 
                                position: token.position, 
                                description: format!("Operator {:?} is not supported for trailing unary operations!", op_token), 
                                error: ParseErrorType::UnsupportedOperator,
                            })
                        }   
                    }?;

                    Ok(Node::<UnaryExpr> {
                        content: UnaryExpr { operator, expr: Box::from(expression), operator_first: false },
                        position: token.position,
                        type_data: TypeData::unknown(),
                    })
                } else {
                    Err(ParseError{ 
                        position: token.position, 
                        description: format!("Expected operator! Found {:?}", token.token_type), 
                        error: ParseErrorType::UnexpectedToken 
                    })
                }
            }
        }
    }
}

struct BinaryExprParseContext {
    pub operator_index : usize
}

impl Parser<BinaryExpr, BinaryExprParseContext> for BinaryExpr {
    fn parse(tokens : &mut Vec<&Token>, context : BinaryExprParseContext) -> Result<Node<BinaryExpr>, ParseError> {
        let position = tokens[0].position;
        let mut before : Vec<&Token> = vec![];
        let mut after : Vec<&Token> = vec![];
        let operator = match tokens[context.operator_index].token_type {
            TokenType::Operator(op_token) => {
                match op_token {
                    OperatorToken::Pipe => todo!(),
                    OperatorToken::Ambersand => todo!(),
                    OperatorToken::Plus => Operator::Plus,
                    OperatorToken::Minus => todo!(),
                    OperatorToken::Star => Operator::Mul,
                    OperatorToken::Exclamation => todo!(),
                    OperatorToken::Assign => todo!(),
                    OperatorToken::Eq => todo!(),
                    OperatorToken::NEq => todo!(),
                    OperatorToken::ThickArrow => todo!(),
                    OperatorToken::ThinArrow => todo!(),
                }
            },
            _ => {
                panic!("Operator is {:?}", tokens[context.operator_index]);
            }
        };

        for i in 0..(context.operator_index) {
            before.push(tokens[i]);
        }
        for i in (context.operator_index + 1)..tokens.len() {
            after.push(tokens[i]);
        }

        let left = Expression::parse(&mut before, ())?;
        let right = Expression::parse(&mut after, ())?;
        
        Ok(Node {
            content: BinaryExpr { operator, left: Box::from(left), right: Box::from(right) },
            position: position,
            type_data: TypeData::unknown(),
        })
    }
}