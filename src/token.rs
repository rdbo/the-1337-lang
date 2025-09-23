use phf::phf_map;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberFormat {
    Decimal,
    Hex,
    Octal,
    Binary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Unknown(String),
    Comment(String),

    // Expressions
    Identifier(String),
    String(String),
    Number {
        value: String,
        format: NumberFormat,
        is_float: bool,
    },

    // Symbols
    SemiColon,

    Colon,
    Assign,
    Walrus,

    LeftParen,
    RightParen,

    LeftCurly,
    RightCurly,

    Plus,
    Minus,
    Times,
    DividedBy,

    Comma,

    // Keywords
    KwExtern,
    KwReturn,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub start_pos: Position,
    pub end_pos: Position,
}

pub static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "extern" => Token::KwExtern,
    "return" => Token::KwReturn
};

pub static SYMBOLS: phf::Map<&'static str, Token> = phf_map! {
    ";" => Token::SemiColon,

    ":" => Token::Colon,
    "=" => Token::Assign,
    ":=" => Token::Walrus,

    "(" => Token::LeftParen,
    ")" => Token::RightParen,

    "{" => Token::LeftCurly,
    "}" => Token::RightCurly,

    "*" => Token::Times,

    "," => Token::Comma
};

pub static NUMBER_FORMATS: phf::Map<char, NumberFormat> = phf_map! {
    'x' => NumberFormat::Hex,
    'o' => NumberFormat::Octal,
    'b' => NumberFormat::Binary,
};
