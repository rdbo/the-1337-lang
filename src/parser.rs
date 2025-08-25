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

macro_rules! let_variant {
    ($token_variant:ident, $ident:ident, $token_info_option_expr:expr) => {
        let Some(Token::$token_variant($ident)) =
            $token_info_option_expr.and_then(|x| Some(x.token.clone()))
        else {
            return Node::Invalid;
        };
    };
}

macro_rules! assure_variant {
    ($token_variant:ident, $token_info_option_expr:expr) => {
        let Some(Token::$token_variant) = $token_info_option_expr.and_then(|x| Some(&x.token))
        else {
            return Node::Invalid;
        };
    };
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

    fn parse_function_type(&mut self) -> Option<Type> {
        let mut params: Vec<(String, Type)> = Vec::new();
        loop {
            // TODO: Deny right parenthesis after comma
            let token_info = self.next_token_info()?.to_owned();
            if let Token::RightParen = token_info.token {
                break;
            }

            let Token::Identifier(identifier) = &token_info.token else {
                return None;
            };
            let Token::Colon = self.next_token_info()?.token else {
                return None;
            };
            let param_type = self.parse_type()?;

            params.push((identifier.to_owned(), param_type));

            let token_info = self.next_token_info()?;
            match token_info.token {
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
        let token_info = self.next_token_info()?;
        let t = match &token_info.token {
            Token::Identifier(s) => Type::Common(s.to_owned()),
            Token::Times => Type::Pointer(Box::new(self.parse_type()?)),
            Token::LeftParen => self.parse_function_type()?,
            _ => return None,
        };

        Some(t)
    }

    pub fn parse_extern(&mut self) -> Node {
        let_variant!(Identifier, identifier, self.next_token_info());
        assure_variant!(Colon, self.next_token_info());
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

        let next_token = self.next_token_info()?;
        let node = match next_token.token {
            Token::KwExtern => self.parse_extern(),
            _ => Node::Invalid,
        };

        // Pop current parsing depth
        let tokens: Vec<TokenInfo> = self.pending_tokens.pop()?;

        Some(NodeInfo { node, tokens })
    }
}
