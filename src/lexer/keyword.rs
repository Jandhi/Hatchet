use std::fmt::Display;



#[derive(Debug)]
pub enum KeyWord {
    If,
    Else,
}

impl KeyWord {
    pub fn get(input: &str) -> Option<KeyWord> {
        match input {
            "if" => Some(KeyWord::If),
            "else" => Some(KeyWord::Else),
            _ => None,
        }
    }
}

impl Display for KeyWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            KeyWord::If => "if",
            KeyWord::Else => "else",
        })
    }
}