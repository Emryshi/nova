// src/lexer.rs
// NovaLang için basit bir lexer (tokenizer)

use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Func,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    Identifier(String),
    Int(i64),
    Float(f64),
    String(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    Comment,
    Eof,
    LBracket,
    RBracket,
    Colon,
    Import,
    Arrow,
    Class,
    Extends,
    New,
    Export,
    Async,
    Await,
    Finally,
    Dot,
}

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            src: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.src.get(self.pos).copied();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        ch
    }

    fn peek_next(&self) -> Option<char> {
        self.src.get(self.pos + 1).copied()
    }

    pub fn next_token(&mut self) -> Token {
        // Burada boşluk atlama, anahtar kelime eşleme, sayı/string/identifier/yorum ayrımı ve hata yönetimi olacak
        match self.peek() {
            Some('[') => return Token { kind: TokenKind::LBracket, line: self.line, col: self.col },
            Some(']') => return Token { kind: TokenKind::RBracket, line: self.line, col: self.col },
            Some('{') => return Token { kind: TokenKind::LBrace, line: self.line, col: self.col },
            Some('}') => return Token { kind: TokenKind::RBrace, line: self.line, col: self.col },
            Some(':') => return Token { kind: TokenKind::Colon, line: self.line, col: self.col },
            Some(',') => return Token { kind: TokenKind::Comma, line: self.line, col: self.col },
            Some('=') => {
                if self.peek_next() == Some('>') {
                    self.advance(); self.advance();
                    return Token { kind: TokenKind::Arrow, line: self.line, col: self.col };
                }
            },
            Some('.') => return Token { kind: TokenKind::Dot, line: self.line, col: self.col },
            _ => Token {
                kind: TokenKind::Eof,
                line: self.line,
                col: self.col,
            },
        }
    }

    // Tokenize etme fonksiyonları buraya eklenecek
} 