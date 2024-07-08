#[derive(Debug, Clone, Copy)]
pub struct Position {
    line : u32,
    column : u32
}

impl Position {
    pub fn new() -> Position {
        Position { line: 1, column: 1 }
    }

    pub fn next_line(&self) -> Position {
        Position { line: self.line + 1, column: 1 }
    }

    pub fn next_column(&self) -> Position {
        Position { line: self.line, column: self.column + 1 }
    }
}