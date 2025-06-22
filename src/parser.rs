// src/parser.rs
// NovaLang için kapsamlı ve modüler parser

use crate::token::{Token, TokenKind};
use crate::ast::{Expr, Stmt, Program};
use crate::error::{NovaError, NovaErrorKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    pub fn parse_program(&mut self) -> Result<Program, NovaError> {
        let mut body = Vec::new();
        while let Some(token) = self.peek() {
            if let TokenKind::Eof = token.kind {
                break;
            }
            let stmt = self.parse_stmt()?;
            body.push(stmt);
        }
        Ok(Program { body })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, NovaError> {
        // Burada let, func, if, while, return, expr, block ayrımı yapılacak
        Err(NovaError::new(NovaErrorKind::ParseError, "Henüz uygulanmadı", 0, 0, None))
    }

    fn parse_list_literal(&mut self) -> Result<Expr, NovaError> {
        let start = self.advance().unwrap(); // '['
        let mut items = Vec::new();
        let mut first = true;
        while let Some(tok) = self.peek() {
            if let TokenKind::RBracket = tok.kind {
                self.advance();
                break;
            }
            if !first {
                self.expect(TokenKind::Comma)?;
            } else {
                first = false;
            }
            let expr = self.parse_expr()?;
            items.push(expr);
        }
        Ok(Expr::List(items, start.line, start.col))
    }

    fn parse_map_literal(&mut self) -> Result<Expr, NovaError> {
        let start = self.advance().unwrap(); // '{'
        let mut pairs = Vec::new();
        let mut first = true;
        while let Some(tok) = self.peek() {
            if let TokenKind::RBrace = tok.kind {
                self.advance();
                break;
            }
            if !first {
                self.expect(TokenKind::Comma)?;
            } else {
                first = false;
            }
            let key = self.parse_expr()?;
            self.expect(TokenKind::Colon)?;
            let value = self.parse_expr()?;
            pairs.push((key, value));
        }
        Ok(Expr::Map(pairs, start.line, start.col))
    }

    fn parse_index_expr(&mut self, collection: Expr) -> Result<Expr, NovaError> {
        let lbracket = self.advance().unwrap(); // '['
        let index = self.parse_expr()?;
        self.expect(TokenKind::RBracket)?;
        Ok(Expr::Index {
            collection: Box::new(collection),
            index: Box::new(index),
            line: lbracket.line,
            col: lbracket.col,
        })
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), NovaError> {
        if let Some(tok) = self.peek() {
            if tok.kind == kind {
                self.advance();
                Ok(())
            } else {
                Err(NovaError::new(NovaErrorKind::ParseError, "Beklenen token bulunamadı", tok.line, tok.col, None))
            }
        } else {
            Err(NovaError::new(NovaErrorKind::ParseError, "Beklenen token bulunamadı", 0, 0, None))
        }
    }

    fn parse_try_catch(&mut self) -> Result<Stmt, NovaError> {
        let try_tok = self.advance().unwrap(); // 'try'
        self.expect(TokenKind::LBrace)?;
        let try_block = self.parse_block()?;
        let catch_tok = self.advance().unwrap(); // 'catch'
        self.expect(TokenKind::LParen)?;
        let err_name = if let Some(Token { kind: TokenKind::Identifier(name), .. }) = self.advance() {
            name
        } else {
            return Err(NovaError::new(NovaErrorKind::ParseError, "catch bloğunda hata değişkeni bekleniyor", catch_tok.line, catch_tok.col, None));
        };
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;
        let catch_block = self.parse_block()?;
        Ok(Stmt::TryCatch {
            try_block,
            err_name,
            catch_block,
            line: try_tok.line,
            col: try_tok.col,
        })
    }

    fn parse_import(&mut self) -> Result<Stmt, NovaError> {
        let import_tok = self.advance().unwrap(); // 'import'
        let name = if let Some(Token { kind: TokenKind::Identifier(name), .. }) = self.advance() {
            name
        } else {
            return Err(NovaError::new(NovaErrorKind::ParseError, "import sonrası modül adı bekleniyor", import_tok.line, import_tok.col, None));
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Import { name, line: import_tok.line, col: import_tok.col })
    }

    fn parse_lambda(&mut self) -> Result<Expr, NovaError> {
        let lparen = self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        let mut first = true;
        while let Some(tok) = self.peek() {
            if let TokenKind::RParen = tok.kind {
                self.advance();
                break;
            }
            if !first {
                self.expect(TokenKind::Comma)?;
            } else {
                first = false;
            }
            if let Some(Token { kind: TokenKind::Identifier(name), .. }) = self.advance() {
                params.push(name);
            } else {
                return Err(NovaError::new(NovaErrorKind::ParseError, "Lambda parametresi bekleniyor", 0, 0, None));
            }
        }
        self.expect(TokenKind::Arrow)?;
        let body = Box::new(self.parse_expr()?);
        Ok(Expr::Lambda {
            params,
            body,
            line: lparen.line,
            col: lparen.col,
        })
    }

    fn parse_class(&mut self) -> Result<Stmt, NovaError> {
        // class SınıfAdı [extends ÜstSınıf] { ... }
        // ...
        Err(NovaError::new(NovaErrorKind::ParseError, "Class parse edilmedi", 0, 0, None))
    }

    fn parse_export(&mut self) -> Result<Stmt, NovaError> {
        // export let/fonksiyon/sınıf
        // ...
        Err(NovaError::new(NovaErrorKind::ParseError, "Export parse edilmedi", 0, 0, None))
    }

    fn parse_async_func(&mut self) -> Result<Stmt, NovaError> {
        // async func ...
        // ...
        Err(NovaError::new(NovaErrorKind::ParseError, "Async func parse edilmedi", 0, 0, None))
    }

    fn parse_await(&mut self) -> Result<Expr, NovaError> {
        // await expr
        // ...
        Err(NovaError::new(NovaErrorKind::ParseError, "Await parse edilmedi", 0, 0, None))
    }

    fn parse_try_catch_finally(&mut self) -> Result<Stmt, NovaError> {
        // try { ... } catch (err) { ... } finally { ... }
        // ...
        Err(NovaError::new(NovaErrorKind::ParseError, "Try-catch-finally parse edilmedi", 0, 0, None))
    }

    fn parse_method_call(&mut self, object: Expr) -> Result<Expr, NovaError> {
        // obj.method(args)
        // ...
        Err(NovaError::new(NovaErrorKind::ParseError, "Method call parse edilmedi", 0, 0, None))
    }

    fn parse_new(&mut self) -> Result<Expr, NovaError> {
        // new SınıfAdı(args)
        // ...
        Err(NovaError::new(NovaErrorKind::ParseError, "New parse edilmedi", 0, 0, None))
    }

    // Diğer yardımcı parse fonksiyonları buraya eklenecek
} 