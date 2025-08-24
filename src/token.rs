#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Unknown(String),
    Comment(String),

    // Expressions
    Identifier(String),
    String(String),

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
}
