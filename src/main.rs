mod pointer;
mod memory;
mod object;
mod flags;

use crate::pointer::Pointer;

fn main() {
    println!("Hello, world!");

    let mut ptr = Pointer::new("test");



    println!("{} {}", ptr, *ptr);
}