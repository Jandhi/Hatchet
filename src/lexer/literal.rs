use std::fmt::Display;


#[derive(Debug)]
pub enum Literal {
    Bool(bool),
    Number(String),
    String(String),
}

impl Literal {
    pub fn get(string: &str) -> Option<Literal> {
        type Lit = Literal;

        match string {
            "true" => Some(Lit::Bool(true)),
            "false" => Some(Lit::Bool(false)),

            _ => None,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Literal::Bool(val) => format!("{}", val),
            Literal::Number(val) => format!("{}", val),
            Literal::String(val) => format!("\"{}\"", val),
        })
    }
}