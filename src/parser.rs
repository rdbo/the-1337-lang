use crate::{Expression, Lexer, Node, Statement, Token, TokenInfo, Type};

pub struct Parser {
    lexer: Lexer,
    pending_tokens: Vec<Vec<TokenInfo>>,
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub node: Node,
    pub tokens: Vec<TokenInfo>,
}

impl Parser {
    pub fn new(content: String) -> Self {
        Self {
            lexer: Lexer::new(content),
            pending_tokens: vec![],
        }
    }

    fn next_token_info(&mut self) -> Option<&TokenInfo> {
        let token = self.lexer.tokenize()?;
        let last_depth = self.pending_tokens.last_mut()?;
        last_depth.push(token);
        last_depth.last()
    }

    fn next_token(&mut self) -> Option<&Token> {
        self.next_token_info().and_then(|x| Some(&x.token))
    }

    fn next_token_owned(&mut self) -> Option<Token> {
        self.next_token().and_then(|x| Some(x.to_owned()))
    }

    fn parse_function_type(&mut self) -> Option<Type> {
        let mut params: Vec<(String, Type)> = Vec::new();
        loop {
            // TODO: Deny right parenthesis after comma
            let token = self.next_token()?.to_owned();
            if matches!(token, Token::RightParen) {
                break;
            }

            let Token::Identifier(identifier) = &token else {
                return None;
            };
            let Token::Colon = self.next_token()? else {
                return None;
            };
            let param_type = self.parse_type()?;

            params.push((identifier.to_owned(), param_type));

            let token = self.next_token()?;
            match token {
                Token::Comma => continue,
                Token::RightParen => break,
                _ => return None,
            }
        }

        // Parse return type
        let return_type = Box::new(self.parse_type()?);

        Some(Type::Function {
            params,
            return_type,
        })
    }

    fn parse_type(&mut self) -> Option<Type> {
        let token = self.next_token()?;
        let t = match token {
            Token::Identifier(s) => Type::Common(s.to_owned()),
            Token::Times => Type::Pointer(Box::new(self.parse_type()?)),
            Token::LeftParen => self.parse_function_type()?,
            _ => return None,
        };

        Some(t)
    }

    pub fn parse_extern(&mut self) -> Node {
        let Some(Token::Identifier(identifier)) = self.next_token_owned() else {
            return Node::Invalid;
        };
        let Some(Token::Colon) = self.next_token() else {
            return Node::Invalid;
        };
        let Some(declared_type) = self.parse_type() else {
            return Node::Invalid;
        };

        Node::Statement(Statement::Extern {
            identifier,
            declared_type,
        })
    }

    pub fn parse(&mut self) -> Option<NodeInfo> {
        // Push new parsing depth for this (sub)node
        self.pending_tokens.push(vec![]);

        let next_token = self.next_token()?;
        let node = match next_token {
            Token::KwExtern => self.parse_extern(),
            _ => Node::Invalid,
        };

        // Pop current parsing depth
        let tokens: Vec<TokenInfo> = self.pending_tokens.pop()?;

        Some(NodeInfo { node, tokens })
    }
}
