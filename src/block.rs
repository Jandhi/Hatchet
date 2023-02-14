use crate::expression::Expression;
use crate::value::Value;

pub struct Block {
    contents : Vec<Box<dyn Expression>>
}

impl Expression for Block {
    fn evaluate(&self) -> Box<dyn Value> {
        let mut value = self.contents[0].evaluate();

        for expr in self.contents[1..].iter() {
            value = expr.evaluate();
        }

        value
    }
}