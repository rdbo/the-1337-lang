use std::collections::HashMap;

use crate::{KEYWORDS, NUMBER_FORMATS, NumberFormat, SYMBOLS, token::Token};

#[derive(Debug, Clone)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    content: String,
    index: usize,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub start_pos: Position,
    pub end_pos: Position,
}

// TODO: Consider not allowing private tokenize functions
//       to return an optional. Instead, they may either
//       return the expected token or Token::Unknown
impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            content,
            index: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self, index: usize) -> Option<char> {
        self.content.chars().nth(index)
    }

    fn peek_offset(&self, offset: usize) -> Option<char> {
        self.peek(self.index + offset)
    }

    fn peek_next(&self) -> Option<char> {
        self.peek_offset(1)
    }

    fn current(&self) -> Option<char> {
        self.peek(self.index)
    }

    fn skip(&mut self) {
        let current = self.current();
        if let Some(c) = current {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else if c != '\r' {
                self.column += 1;
            }
        }

        self.index += 1;
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.current()?;
        self.skip();

        Some(c)
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.current() {
            if !c.is_whitespace() {
                break;
            }

            self.skip();
        }
    }

    fn position(&self) -> Position {
        Position {
            index: self.index,
            line: self.line,
            column: self.column,
        }
    }

    fn tokenize_identifier_or_keyword(&mut self) -> Option<Token> {
        let mut ident = String::new();
        while let Some(c) = self.current() {
            match c {
                'a'..'z' | 'A'..'Z' | '0'..'9' | '_' => {
                    ident.push(c);
                    self.skip();
                    continue;
                }
                _ => break,
            };
        }

        KEYWORDS
            .get(ident.as_str())
            .map(|token| token.to_owned())
            .or_else(|| Some(Token::Identifier(ident)))
    }

    fn tokenize_string(&mut self) -> Option<Token> {
        let mut quote_count: usize = 0;
        while let Some(c) = self.current() {
            if c != '"' {
                break;
            }

            quote_count += 1;
            self.skip();
        }

        if quote_count == 2 {
            return Some(Token::String("".to_owned()));
        } else if quote_count & 1 == 0 {
            // Even-lengthed long quotes not allowed!
            // They are ambiguous.
            return Some(Token::Unknown('"'.to_string().repeat(quote_count)));
        }

        let mut end_quote_count = 0;
        let mut string_content = String::new();
        while end_quote_count != quote_count {
            let Some(c) = self.advance() else {
                return Some(Token::Unknown(format!(
                    "{}{}",
                    '"'.to_string().repeat(quote_count),
                    string_content
                )));
            };

            string_content.push(c);

            if c == '"' {
                end_quote_count += 1;
            } else {
                end_quote_count = 0;
            }
        }

        // Remove trailing double quotes from string content
        string_content.truncate(string_content.len() - end_quote_count);

        Some(Token::String(string_content))
    }

    fn tokenize_number(&mut self) -> Option<Token> {
        let mut value = String::new();
        let mut format = NumberFormat::Decimal;
        let mut is_float = false;
        while let Some(c) = self.current() {
            match c {
                'x' | 'X' | 'b' | 'B' | 'o' | 'O' if value == "0" => {
                    format = NUMBER_FORMATS
                        .get(&c.to_ascii_lowercase())
                        .expect(format!("Missing expected number format: {}", c).as_str())
                        .to_owned();
                    self.skip();
                    continue;
                }
                '.' if !is_float => {
                    is_float = true;
                }
                '0' | '1' => {}
                '2'..='7' if format != NumberFormat::Binary => {}
                '8'..='9' if format != NumberFormat::Binary && format != NumberFormat::Octal => {}
                'a'..='f' | 'A'..'F' if format == NumberFormat::Hex => {}
                _ => {
                    // TODO: Handle invalid numbers
                    break;
                }
            }

            value.push(c);
            self.skip();
        }

        Some(Token::Number {
            value,
            format,
            is_float,
        })
    }

    fn try_tokenize_symbol(&mut self) -> Option<Token> {
        let mut symbol_str = String::new();
        let mut best_match = None;
        loop {
            let Some(c) = self.current() else {
                break;
            };

            symbol_str.push(c);
            let Some(symbol) = SYMBOLS.get(&symbol_str) else {
                break;
            };

            best_match = Some(symbol.to_owned());
            self.skip();
        }

        best_match
    }

    pub fn tokenize(&mut self) -> Option<TokenInfo> {
        self.skip_whitespaces();
        let c = self.current()?;
        let start_pos = self.position();

        let symbol_token = self.try_tokenize_symbol();

        let token = match c {
            _ if symbol_token.is_some() => symbol_token.unwrap(),
            '"' => self.tokenize_string()?,
            '0'..='9' => self.tokenize_number()?,
            'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier_or_keyword()?,
            _ => {
                self.skip();
                Token::Unknown(c.to_string())
            }
        };

        let end_pos = self.position();

        Some(TokenInfo {
            token,
            start_pos,
            end_pos,
        })
    }
}
