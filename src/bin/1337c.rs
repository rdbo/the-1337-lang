use std::fs::{self};

use the_1337_lang::*;

fn main() {
    let source_code = fs::read_to_string("example/first.1337").expect("failed to read source file");
    let mut parser = Parser::new(source_code);

    println!("Parsing...");
    while let Some(node) = parser.parse() {
        println!("{:?}", node);
    }
    println!("Done");
}
