use crate::{Expression, Lexer, Node, Statement, Token, TokenInfo, Type};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

pub struct Context {
    index_start: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn skip(&mut self, count: usize) {
        self.index += count;
    }

    fn peek(&self, index: usize) -> Option<Token> {
        self.tokens.get(index).map(|x| x.to_owned())
    }

    fn current(&self) -> Option<Token> {
        self.peek(self.index)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.current()?;
        self.skip(1);
        Some(token)
    }

    pub fn parse(&mut self) -> Option<Node> {
        None
    }
}
