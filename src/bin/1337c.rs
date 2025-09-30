use std::fs::{self};

use the_1337_lang::*;

fn main() {
    let source_code = fs::read_to_string("example/first.1337").expect("failed to read source file");

    println!("Tokenizing....");

    println!("--------------------------------");
    let mut lexer = Lexer::new(source_code);
    let mut tokens: Vec<TokenInfo> = vec![];
    for i in 0.. {
        let Some(token) = lexer.tokenize() else {
            break;
        };
        println!("{}: {:?}", i, token);
        tokens.push(token);
    }
    println!("--------------------------------");

    println!();

    println!("Parsing...");
    println!("================================");

    let mut parser = Parser::new(tokens);
    while let Some(node) = parser.parse() {
        println!("{:#?}", node);
        // if let Node::Invalid = node.node {
        //     let parsed = &parser.tokens()[node.start_index..node.end_index];
        //     println!("Bad parse: {:?}", parsed);
        // }
    }

    println!("================================");

    println!();

    println!("Done");
}
