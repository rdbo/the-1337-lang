use crate::{Expression, Lexer, Node, Statement, Token, TokenInfo};

pub struct Parser {
    lexer: Lexer,
    current_token_info: Option<TokenInfo>,
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub node: Node,
    pub index_start: usize,
    pub index_end: usize,
    pub line_start: usize,
    pub column_start: usize,
    pub line_end: usize,
    pub column_end: usize,
}

impl Parser {
    fn new(content: String) -> Self {
        Self {
            lexer: Lexer::new(content),
            current_token_info: None,
        }
    }

    fn next_token_info(&mut self) -> Option<TokenInfo> {
        self.current_token_info = self.lexer.tokenize();
        self.current_token_info.clone()
    }

    fn parse_declaration(&self) -> Option<Expression> {
        None
    }

    fn parse_extern(&mut self) -> Option<Statement> {
        None
    }

    fn parse(&mut self) -> Option<NodeInfo> {
        None
    }
}
