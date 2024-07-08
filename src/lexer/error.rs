use crate::position::Position;


#[derive(Debug)]
pub struct LexError {
    pub error: LexErrorType,
    pub description: String,
    pub position: Position,
}

#[derive(Debug)]
pub enum LexErrorType {
    UnexpectedSymbol,
    UnrecognizedOperator,
}
