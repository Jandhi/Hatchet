
use Hatchet::{input::read_file, lexer::lexer::lex};



fn main() {
    println!("--- READING FILE ---");
    let content = read_file("test/test.hat");
    println!("{}", content);

    println!("--- LEXING ---");
    let lexed = lex(&content);
    println!("{:?}", lexed);
}
