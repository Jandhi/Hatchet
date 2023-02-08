enum TokenType {
    Identifier,
    Keyword,
    Separator,
    Operator,
    Literal,
    Comment
}

pub struct Token {
    tokenType : TokenType,
    contents : String,
}