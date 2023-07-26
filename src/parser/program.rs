





use super::{function::{function::Function}, statement::Statement, context::Context};

pub struct Program {
    pub functions : Vec<Function>,
    pub main : Vec<Statement>,
}


pub trait CodeWriter {
    fn write(&self, buffer : &mut String, context : &mut Context);
}

impl CodeWriter for Program {
    fn write(&self, buffer : &mut String, context : &mut Context) {
        let mut pre = String::from("");
        let mut main = String::from("");

        pre.push_str("#include<iostream>\n");

        for function in &self.functions {
            if function.used {
                function.write(&mut pre,  context);
            }
        };

        for statement in &self.main {
            statement.write(&mut main,  context);
            main.push(';');
            main.push('\n');
        }

        buffer.push_str(format!(
            "{}\nint main() {{\n{}}}", pre, main).as_str());
    }
}