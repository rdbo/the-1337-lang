use std::collections::HashMap;

use crate::token::Token;

pub struct Lexer {
    content: String,
    index: usize,
    line: usize,
    column: usize,
    current_char: Option<char>,
}

#[derive(Debug, Clone)]
pub struct TokenContext {
    pub source_file: String,
    pub index_start: usize,
    pub index_end: usize,
    pub line_start: usize,
    pub column_start: usize,
    pub line_end: usize,
    pub column_end: usize,
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub context: TokenContext,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            content,
            index: 0,
            line: 1,
            column: 1,
            current_char: None,
        }
    }

    fn read_next(&mut self) -> Option<char> {
        self.current_char = None;
        if self.index >= self.content.len() {
            return self.current_char;
        }

        let c = self.content.chars().nth(self.index)?;
        self.index += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else if c != '\r' {
            self.column += 1;
        }

        self.current_char = Some(c);
        self.current_char.clone()
    }

    fn skip_whitespaces(&mut self) {
        if self.current_char.is_some_and(|c| !c.is_whitespace()) {
            return;
        }

        while let Some(c) = self.read_next() {
            if !c.is_whitespace() {
                break;
            }
        }
    }

    fn tokenize_identifier_or_keyword(&mut self) -> Option<Token> {
        let keywords: HashMap<&str, Token> = HashMap::from([("extern", Token::KwExtern)]);
        let mut ident = self.current_char?.to_string();
        while let Some(c) = self.read_next() {
            match c {
                'a'..'z' | 'A'..'Z' | '0'..'9' | '_' => ident.push(c),
                _ => break,
            };
        }

        Some(
            keywords
                .get(ident.as_str())
                .map(|token| token.to_owned())
                .unwrap_or_else(|| Token::Identifier(ident)),
        )
    }

    pub fn tokenize(&mut self) -> Option<TokenInfo> {
        self.skip_whitespaces();
        let c = self.current_char?;
        let mut context = TokenContext {
            source_file: "<memory>".to_owned(),
            index_start: self.index - 1,
            index_end: self.index,
            line_start: self.line,
            column_start: self.column,
            line_end: self.line,
            column_end: self.column + 1,
        };

        let token = match c {
            ';' => Token::SemiColon,
            ':' => {
                self.read_next();
                self.skip_whitespaces();
                if let Some('=') = self.current_char {
                    Token::Walrus
                } else {
                    Token::Colon
                }
            }
            '=' => Token::Assign,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftCurly,
            '}' => Token::RightCurly,
            'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier_or_keyword()?,
            _ => Token::Unknown(c.to_string()),
        };

        context.line_end = self.line;
        context.column_end = self.column + 1;

        // Some symbols do not run read_next(),
        // lets do that here.
        // TODO: Figure out a cleaner solution
        if context.index_end == self.index {
            self.read_next();
        }

        Some(TokenInfo { token, context })
    }
}
