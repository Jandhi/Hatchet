use crate::{lexer::lexemes::Lexeme, parser::function::call::Call};

use super::{function::function::Function, expression::Expression, program::Program};

pub struct Parser {
    pub functions : Vec<Function>,
    pub main : Vec<Expression>,
}

impl Parser {
    pub fn parse(mut self, lexemes : &mut Vec<Lexeme>) -> Program {
        let mut line = vec![];

        while lexemes.len() > 0 {
            let next = lexemes.remove(0);
            
            match next {
                Lexeme::Newline => {
                    if line.len() > 0 {
                        self.main.push(self.parse_line(&mut line));
                    }
                }
                _ => {
                    line.push(next);
                }
            };
        }

        if line.len() > 0 {
            self.main.push(self.parse_line(&mut line));
        }

        self.check_calls();

        Program {
            functions: self.functions,
            main: self.main,
        }
    }

    fn parse_line(&self, mut lexemes : &mut Vec<Lexeme>) -> Expression {
        assert!(lexemes.len() > 0, "empty lines should be ignored");
        println!("Parsing line: {:?}", lexemes);

        let mut lexeme_lists = vec![];
        let mut curr_list = vec![];

        for lexeme in lexemes {
            match lexeme {
                Lexeme::Pipe => {
                    lexeme_lists.push(curr_list.clone());
                    curr_list = vec![];
                },
                _ => {
                    curr_list.push(lexeme.clone());
                }
            }
        }

        lexeme_lists.push(curr_list);

        let mut expr = self.parse_expression_or_call(&mut lexeme_lists[0]);
        

        for i in 1..lexeme_lists.len() {
            expr = self.parse_piped_call(&mut lexeme_lists[i], expr);
        };

        expr
    }

    fn parse_expression_or_call(&self, mut lexemes : &mut Vec<Lexeme>) -> Expression {
        let first = lexemes.pop().unwrap();
        match first {
            Lexeme::Literal(literal) => return Expression::Literal(literal),
            _ => {
                lexemes.insert( 0, first);
                self.parse_call(lexemes)
            }
        }
    }

    fn parse_call(&self, mut lexemes : &mut Vec<Lexeme>) -> Expression {
        let first = lexemes.pop().unwrap();

        match first {
            Lexeme::Identifier(identifier) => {
                let mut args = vec![];

                while lexemes.len() > 0 {
                    args.push(self.parse_expression(lexemes))
                }

                Expression::FunctionCall(Call{
                    func_name: identifier,
                    args: args,
                })                
            },
            _ => todo!()
        }
    }

    fn parse_piped_call(&self, mut lexemes : &mut Vec<Lexeme>, piped_value : Expression) -> Expression {
        let first = lexemes.pop().unwrap();

        match first {
            Lexeme::Identifier(identifier) => {
                let mut args = vec![piped_value];

                while lexemes.len() > 0 {
                    args.push(self.parse_expression(lexemes))
                }

                Expression::FunctionCall(Call{
                    func_name: identifier,
                    args: args,
                })                
            },
            _ => todo!("{:?}", first)
        }
    }

    fn parse_expression(&self, mut lexemes : &mut Vec<Lexeme>) -> Expression
    {
        let first = lexemes.pop().unwrap();

        match first {
            Lexeme::Literal(literal) => Expression::Literal(literal),
            _ => todo!(),
        }
    }

    fn parse_parens(&mut self, mut lexemes : &mut Vec<Lexeme>) -> Expression {
        todo!()
    }

    fn check_calls(&mut self) {
        let mut functions  = vec![];

        for func in &mut self.functions {
            functions.push(func);
        }

        for expr in &self.main {
            check_for_calls(expr, &mut functions);
        }
    }

    
}

fn check_for_calls(expr : &Expression, functions : &mut Vec<&mut Function>) {
    match expr {
        Expression::FunctionCall(call) => {
            register_call(call, functions)
        },
        Expression::Literal(_) => {}, // okay!
    }
}

fn register_call(call : &Call, functions : &mut Vec<&mut Function>) {
    let mut found = false;

    for func in &mut *functions {
        let left : &str = &call.func_name;
        let right : &str = &func.name;
        if  left == right {
            // match!
            func.used = true;
            found = true;
            break;
        }
    }

    assert!(found, "No matching function found for {:?}", call);

    for expr in &call.args {
        check_for_calls(expr, functions);
    }
}