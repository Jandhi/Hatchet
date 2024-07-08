use std::fmt::Display;


#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Pipe,
    Ambersand,

    Plus,
    Minus,
    Star,

    Exclamation,

    Assign,
    Eq,
    NEq,

    ThickArrow,
    ThinArrow,
}

impl Operator {
    pub fn get(string: &str) -> Option<Operator> {
        type Op = Operator;
        match string {
            "|" => Some(Op::Pipe),
            "&" => Some(Op::Ambersand),

            "+" => Some(Op::Plus),
            "-" => Some(Op::Minus),
            "*" => Some(Op::Star),

            "!" => Some(Op::Exclamation),

            "=" => Some(Op::Assign),
            "==" => Some(Op::Eq),
            "!=" => Some(Op::NEq),

            "->" => Some(Op::ThinArrow),
            "=>" => Some(Op::ThickArrow),

            _ => None,
        }
    }

    pub fn precedence(&self) -> u32 {
        match self {
            Operator::Pipe => 10,
            Operator::Ambersand => 10,
            Operator::Plus => 5,
            Operator::Minus => 5,
            Operator::Star => 8,
            Operator::Exclamation => 12,
            Operator::Assign => 2,
            Operator::Eq => 3,
            Operator::NEq => 3,
            Operator::ThickArrow => 1,
            Operator::ThinArrow => 1,
        }
    }

    pub fn is_higher_precedence(&self, other : Operator) -> bool {
        self.precedence() > other.precedence()
    }
}


impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operator::Pipe => "|",
            Operator::Ambersand => "&",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Exclamation => "!",
            Operator::Assign => "=",
            Operator::Eq => "==",
            Operator::NEq => "!=",
            Operator::ThickArrow => "=>",
            Operator::ThinArrow => "->",
        })
    }
}