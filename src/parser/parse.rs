use crate::lexer::token::Token;
use crate::lexer::token::TokenType;
use crate::scope::Scope;

pub fn parse(mut tokens : Vec<Token>, mut localScope : &Scope, globalScope : &Scope) {
    match tokens[0].token_type {
        TokenType::Identifier => {
            let token = tokens.pop().unwrap();
            parse_identifier(&token.contents, tokens, localScope, globalScope)
        },
        _ => {
            
        },
    }
}

pub fn parse_identifier(contents : &String, mut tokens : Vec<Token>, mut localScope : &Scope, globalScope : &Scope) {
    let end_index = tokens.iter().position(|token| matches!(token.token_type, TokenType::Identifier));

    let mut args : Vec<Token> = vec![];   

    if let Some(index) = end_index {
        for _ in 0..index {
            args.push(tokens.pop().unwrap());
        }
    } else {
        while let Some(token) = tokens.pop() {
            args.push(token);
        }
    }

    
}