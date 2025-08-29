use std::fs::{self};

use the_1337_lang::*;

fn main() {
    let source_code = fs::read_to_string("example/first.1337").expect("failed to read source file");

    println!("Tokenizing....");

    println!("--------------------------------");
    let mut lexer = Lexer::new(source_code);
    let tokens: Vec<TokenInfo> = vec![];
    while let Some(token) = lexer.tokenize() {
        println!("{:?}", token);
    }
    println!("--------------------------------");

    println!();

    // println!("Parsing...");
    // println!("================================");

    // let mut parser = Parser::new(tokens);
    // while let Some(node) = parser.parse() {
    //     println!("{:?}", node);
    // }

    // println!("================================");

    // println!();

    println!("Done");
}
