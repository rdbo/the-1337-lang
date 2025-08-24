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

    // Keywords
    KwExtern,
    KwReturn,
}
