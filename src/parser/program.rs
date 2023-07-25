



use super::{function::{function::Function}, statement::Statement, context::WriterContext};

pub struct Program {
    pub functions : Vec<Function>,
    pub main : Vec<Statement>,
}


pub trait CodeWriter {
    fn write(&self, buffer : &mut String, program : &Program, context : &WriterContext);
}

impl CodeWriter for Program {
    fn write(&self, buffer : &mut String, _program : &Program, context : &WriterContext) {
        let mut pre = String::from("");
        let mut main = String::from("");

        pre.push_str("#include<iostream>\n");

        for function in &self.functions {
            if function.used {
                function.write(&mut pre, &self, context);
            }
        };

        for statement in &self.main {
            statement.write(&mut main, &self, context);
            main.push(';');
            main.push('\n');
        }

        buffer.push_str(format!(
            "{}\nint main() {{\n{}}}", pre, main).as_str());
    }
}