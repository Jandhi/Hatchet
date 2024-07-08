use crate::position::Position;

#[derive(Debug)]
pub struct ParseError {
    pub position : Position,
    pub description : String,
    pub error : ParseErrorType,
}

#[derive(Debug)]
pub enum ParseErrorType {
    UnexpectedTabAmount,
    UnexpectedToken,
    UnsupportedOperator,
    NumberFormat,
    InvalidExpression,
    MalformedParentheses,
}