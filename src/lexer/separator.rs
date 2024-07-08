use std::fmt::Display;


#[derive(Debug)]
pub enum Separator {
    Tab,
    Newline,

    Comma,
    Period,
    Colon,

    // BRACKETS
    LeftBracket,
    RightBracket,

    LeftSquareBracket,
    RightSquareBracket,

    LeftTriangleBracket,
    RightTriangleBracket,

    LeftParen,
    RightParen,
}

impl Separator {
    pub fn get(string: &str) -> Option<Separator> {
        type Sep = Separator;
        match string {
            "." => Some(Sep::Period),
            "," => Some(Sep::Comma),
            ":" => Some(Sep::Colon),

            "\t" => Some(Sep::Tab),
            "\n" => Some(Sep::Newline),

            "{" => Some(Sep::LeftBracket),
            "}" => Some(Sep::RightBracket),

            "[" => Some(Sep::LeftSquareBracket),
            "]" => Some(Sep::RightSquareBracket),

            "<" => Some(Sep::LeftTriangleBracket),
            ">" => Some(Sep::RightTriangleBracket),

            "(" => Some(Sep::LeftParen),
            ")" => Some(Sep::RightParen),

            _ => None,
        }
    }
}

impl Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Separator::Tab => "\t",
            Separator::Newline => "\n",
            Separator::Comma => ",",
            Separator::Period => ".",
            Separator::Colon => ":",
            Separator::LeftBracket => "{",
            Separator::RightBracket => "}",
            Separator::LeftSquareBracket => "[",
            Separator::RightSquareBracket => "]",
            Separator::LeftTriangleBracket => "<",
            Separator::RightTriangleBracket => ">",
            Separator::LeftParen => "(",
            Separator::RightParen => ")",
        })
    }
}