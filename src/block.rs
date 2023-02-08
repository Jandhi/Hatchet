use crate::expression::Expression;

pub struct Block {
    contents : Vec<Box<dyn Expression>>
}

impl Expression for Block {
    fn evaluate(&mut self) -> crate::value::Value {
        let mut value = self.contents[0].evaluate();

        for expr in self.contents[1..].iter() {
            value = expr.evaluate();
        }

        value
    }
}