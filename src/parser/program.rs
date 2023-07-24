use std::fmt::Write;

use super::{function::{function::Function}, expression::Expression};

pub struct Program {
    pub functions : Vec<Function>,
    pub main : Vec<Expression>,
}

pub trait Writer {
    fn write(&self, buffer : &mut String, program : &Program);
}

impl Writer for Program {
    fn write(&self, buffer : &mut String, program : &Program) {
        let mut pre = String::from("");
        let mut main = String::from("");

        pre.push_str("#include<iostream>\n");

        for function in &self.functions {
            if function.used {
                function.write(&mut pre, &self);
            }
        };

        for expr in &self.main {
            expr.write(&mut main, &self);
            main.push(';');
            main.push('\n');
        }

        buffer.push_str(format!(
            "{}
            int main() {{
                {}
            }}
            ", pre, main).as_str());
    }
}