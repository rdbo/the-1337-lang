use std::fs::{self};

use the_1337_lang::*;

fn main() {
    let source_code = fs::read_to_string("example/first.1337").expect("failed to read source file");
    let mut lexer = Lexer::new(source_code);

    println!("Tokenizing...");
    while let Some(token) = lexer.tokenize() {
        println!("{:?}", token);
    }
}
