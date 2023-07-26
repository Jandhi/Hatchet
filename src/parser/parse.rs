use crate::{lexer::lexemes::Lexeme, parser::{function::call::Call, assignment::Assignee}, my_types::Text, types::{hatchet_type::HatchetType::Unknown, type_checker::TypeChecker}};

use super::{function::function::Function, expression::{Expression, ExpressionType}, program::Program, statement::Statement, call_checker::CheckCalls, context::{make_empty_context}};

pub struct Parser {
    pub functions : Vec<Function>,
    pub main : Vec<Statement>,
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
                        line = vec![];
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


        // Type check here
        let mut context = make_empty_context();
        for func in &self.functions {
            context.functions.push(func);
        }
        for statement in &mut self.main {
            statement.check_type(&mut context);
        }

        self.check_calls();

        Program {
            functions: self.functions,
            main: self.main,
        }
    }

    fn parse_line(&self, lexemes : &mut Vec<Lexeme>) -> Statement {
        assert!(lexemes.len() > 0, "empty lines should be ignored");
        println!("Parsing line: {:?}", lexemes);

        if lexemes.contains(&Lexeme::Assignment) {

            match lexemes.remove(0) {
                Lexeme::Identifier(name) => {
                    lexemes.remove(0); // assignment

                    return Statement::Assignment(Assignee::Single(name), self.parse_expression(lexemes))
                }
                lexeme => panic!("Assignment must start with identifier! Started with {:?}", lexeme),
            }

        }

        Statement::Expression(self.parse_expression(lexemes))
    }

    fn parse_pipes(&self, lexemes : &mut Vec<Lexeme>) -> Expression {
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

        let mut expr = self.parse_expression(&mut lexeme_lists[0]);
        

        for i in 1..lexeme_lists.len() {
            expr = self.parse_piped_call(&mut lexeme_lists[i], expr);
        };

        expr
    }

    fn parse_expression(&self, lexemes : &mut Vec<Lexeme>) -> Expression {
        if lexemes.contains(&Lexeme::Pipe) {
            return self.parse_pipes(lexemes);
        }

        let first = lexemes.remove(0);
        match first {
            Lexeme::Literal(literal) => return ExpressionType::Literal(literal).as_expr(),
            Lexeme::Identifier(identity) => {
                self.parse_identifer(identity, lexemes)
            },
            _ => {
                todo!("Can't match other identifiers yet")
            }
        }
    }

    fn parse_identifer(&self, identifier : Text, lexemes : &mut Vec<Lexeme>) -> Expression {
        let mut args = vec![];

        if lexemes.len() == 0 {
            return ExpressionType::VariableRead(identifier).as_expr();
        }

        while lexemes.len() > 0 {
            args.push(self.parse_expression(lexemes))
        }
        
        ExpressionType::FunctionCall(Call{
            func_name: identifier,
            args: args,
            my_type: Unknown,
        }).as_expr()
    }

    fn parse_piped_call(&self, lexemes : &mut Vec<Lexeme>, piped_value : Expression) -> Expression {
        let first = lexemes.remove(0);

        match first {
            Lexeme::Identifier(identifier) => {
                let mut args = vec![piped_value];

                while lexemes.len() > 0 {
                    args.push(self.parse_expression(lexemes))
                }

                ExpressionType::FunctionCall(Call{
                    func_name: identifier,
                    args: args,
                    my_type: Unknown,
                }).as_expr()                
            },
            _ => todo!("{:?}", first)
        }
    }

    fn check_calls(&mut self) {
        for expr in &mut self.main {
            expr.check_for_calls(&mut self.functions)
        }
    }

    
}
