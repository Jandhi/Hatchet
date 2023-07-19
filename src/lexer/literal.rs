use super::lexemes::Text;

#[derive(Debug)]
pub enum Literal {
    String(Text),
    Int32(i32),
    UInt32(u32),
}