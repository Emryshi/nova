#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Anahtar kelimeler
    Let, Func, If, Else, While, Return, True, False, Null,
    // Semboller
    Identifier(String),
    Int(i64),
    Float(f64),
    String(String),
    // Operatörler ve parantezler
    LParen, RParen, LBracket, RBracket, LBrace, RBrace, Comma, Semicolon,
    Assign, Plus, Minus, Star, Slash, Percent,
    Eq, Neq, Lt, Gt, Le, Ge,
    Colon,
    // Yorum ve dosya sonu
    Comment(String),
    Eof,
    Import,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
} 