use crate::my_types::Text;


pub struct WriterContext<'a> {
    variables : Vec<Text>,
    parent_contexts : Vec<&'a WriterContext<'a>>
}

pub fn make_context() -> WriterContext<'static> {
    WriterContext { variables: vec![], parent_contexts: vec![] }
}