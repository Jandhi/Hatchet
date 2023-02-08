pub enum TokenType {
    Identifier,
    Keyword,
    Separator,
    Operator,
    Literal,
    Comment
}

pub struct Token {
    pub token_type : TokenType,
    pub contents : String,
}