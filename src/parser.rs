use crate::{Expression, FunctionParam, Lexer, Node, Statement, Token, TokenInfo, Type};

pub struct Parser {
    tokens: Vec<TokenInfo>,
    index: usize,
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub node: Node,
    pub start_index: usize,
    pub end_index: usize,
    pub message: String,
}

macro_rules! advance_expected {
    ($parser:ident, $variant:ident) => {
        let old_index = $parser.index;
        $parser.skip(1);
        let token_info = $parser.peek_token_info(old_index)?;
        let Token::$variant = token_info.token.to_owned() else {
            return Err(format!("({}@{}:{}) expected {} at '{:?}', found: {:?}", file!(), line!(), column!(), stringify!($variant), token_info.start_pos, token_info.token));
        };
    };

    ($parser:ident, $variant:ident, $($field:ident),+) => {
        let old_index = $parser.index;
        $parser.skip(1);
        let token_info = $parser.peek_token_info(old_index)?;
        let Token::$variant($($field),+) = token_info.token.to_owned() else {
            return Err(format!("({}@{}:{}) expected {} at '{:?}', found: {:?}", file!(), line!(), column!(), stringify!($variant), token_info.start_pos, token_info.token));
        };
    };
}

macro_rules! unexpected_token {
    ($token_info:ident) => {
        return Err(format!(
            "({}@{}:{}) unexpected token at '{:?}': {:?}",
            file!(),
            line!(),
            column!(),
            $token_info.start_pos,
            $token_info.token
        ))
    };
}

#[allow(dead_code)] // Allow helper functions to exist without warnings
impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn tokens(&self) -> &Vec<TokenInfo> {
        &self.tokens
    }

    fn skip(&mut self, count: usize) {
        self.index += count;
    }

    fn peek_token_info(&self, index: usize) -> Result<&TokenInfo, String> {
        self.tokens
            .get(index)
            .ok_or(format!("failed to peek token info at index: {}", index))
    }

    fn current_token_info(&self) -> Result<&TokenInfo, String> {
        self.peek_token_info(self.index)
    }

    fn advance_token_info(&mut self) -> Result<&TokenInfo, String> {
        let old_index = self.index;
        self.skip(1);
        self.peek_token_info(old_index)
    }

    fn peek(&self, index: usize) -> Result<Token, String> {
        self.peek_token_info(index)
            .map(|info| info.token.to_owned())
    }

    fn current(&self) -> Result<Token, String> {
        self.peek(self.index)
    }

    fn advance(&mut self) -> Result<Token, String> {
        let old_index = self.index;
        self.skip(1);
        self.peek(old_index)
    }

    fn parse_function_params(&mut self) -> Result<Vec<FunctionParam>, String> {
        let mut params: Vec<FunctionParam> = vec![];
        loop {
            let token = self.current()?;
            if let Token::RightParen = token {
                self.skip(1);
                break;
            };

            advance_expected!(self, Identifier, name);
            advance_expected!(self, Colon);

            let declared_type = self.parse_type()?;
            params.push(FunctionParam {
                name,
                declared_type,
            });
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let token_info = self.advance_token_info()?;
        let result = match &token_info.token {
            Token::Identifier(ident) => Ok(Type::Common(ident.to_owned())),
            Token::Times => Ok(Type::Pointer(Box::new(self.parse_type()?))),
            Token::LeftParen => {
                let params = self.parse_function_params()?;
                let return_type = Box::new(self.parse_type()?);
                Ok(Type::Function {
                    params,
                    return_type,
                })
            }
            _ => unexpected_token!(token_info),
        };
        result
    }

    fn parse_declaration(&mut self, ident: String) -> Result<Node, String> {
        let declared_type = self.parse_type()?;
        advance_expected!(self, SemiColon);

        Ok(Node::Expression(Expression::Declare {
            identifier: ident,
            declared_type,
        }))
    }

    fn parse_identifier(&mut self, ident: String) -> Result<Node, String> {
        let token_info = self.advance_token_info()?;

        match token_info.token {
            Token::Colon => self.parse_declaration(ident),
            _ => unexpected_token!(token_info),
        }
    }

    fn parse_extern(&mut self) -> Result<Node, String> {
        advance_expected!(self, Identifier, ident);
        advance_expected!(self, Colon);
        let decl_type = self.parse_type()?;
        advance_expected!(self, SemiColon);

        Ok(Node::Statement(Statement::Extern {
            identifier: ident,
            declared_type: decl_type,
        }))
    }

    // Puts the parser in a good spot
    // after parsing a bad node
    fn resynchronize(&mut self) {
        while let Ok(token) = self.current() {
            self.skip(1);
            if let Token::SemiColon = token {
                break;
            }
        }
    }

    pub fn parse(&mut self) -> Option<NodeInfo> {
        let start_index = self.index;
        let token_info = self.advance_token_info().ok()?;

        let result = match token_info.token.to_owned() {
            Token::KwExtern => self.parse_extern(),
            Token::Identifier(ident) => self.parse_identifier(ident),
            _ => Err(format!(
                "invalid root token at '{:?}': {:?}",
                token_info.start_pos, token_info.token
            )),
        };

        let end_index = self.index;

        Some(result.map_or_else(
            |err| {
                self.resynchronize();

                NodeInfo {
                    node: Node::Invalid,
                    start_index,
                    end_index: self.index,
                    message: err,
                }
            },
            |node| NodeInfo {
                node,
                start_index,
                end_index,
                message: "".to_owned(),
            },
        ))
    }
}
