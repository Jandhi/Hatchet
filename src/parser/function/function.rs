use std::rc::Rc;

use crate::{my_types::Text, types::hatchet_type::HatchetType, parser::{expression::Expression, program::{Writer, Program}}};

pub struct Function {
    pub name : Text,
    pub arguments : Vec<HatchetType>,
    pub definition : FunctionDefinition,
    pub return_type : HatchetType,
    pub used : bool
}

pub fn get_function_cpp_name(name : Text) -> Text {
    Rc::from(format!("fn_{}", name).as_str())
}

impl Writer for Function {
    fn write(&self, buffer : &mut String, program : &Program) {
        let name = get_function_cpp_name(self.name.clone());
        let mut args = String::from("");

        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                args.push_str(", ");
            }

            arg.write(&mut args, program);
            args.push_str(" ");
            args.push_str(format!("{}", get_arg_name(i)).as_str());
        }

        let mut return_val = String::from("");
        self.return_type.write(&mut return_val, program);

        buffer.push_str(format!("{} {}({}) {{\n", return_val, name, args).as_str());
        match &self.definition {
            FunctionDefinition::UserDefined(_) => todo!(),
            FunctionDefinition::Predefined(code) => buffer.push_str(&code),
        };
        buffer.push_str("}\n");
    }
}

pub enum FunctionDefinition {
    UserDefined(Expression),
    Predefined(Text),
}

pub fn get_arg_name(index : usize) -> Text {
    return Rc::from(format!("arg{}", index))
}