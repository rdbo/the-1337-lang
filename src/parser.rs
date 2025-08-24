use crate::{Lexer, SyntaxNode};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    fn new(content: String) -> Self {
        Self {
            lexer: Lexer::new(content),
        }
    }

    fn parse() -> Option<SyntaxNode> {
        None
    }
}
