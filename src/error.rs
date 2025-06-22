// src/error.rs
// NovaLang için kapsamlı hata yönetimi

#[derive(Debug, Clone)]
pub enum NovaErrorKind {
    LexError,
    ParseError,
    SemanticError,
    RuntimeError,
}

#[derive(Debug, Clone)]
pub struct NovaError {
    pub kind: NovaErrorKind,
    pub message: String,
    pub line: usize,
    pub col: usize,
    pub snippet: Option<String>,
    pub token: Option<String>,
}

impl NovaError {
    pub fn new(kind: NovaErrorKind, message: &str, line: usize, col: usize, snippet: Option<String>) -> Self {
        NovaError {
            kind,
            message: message.to_string(),
            line,
            col,
            snippet,
            token: None,
        }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn display(&self) {
        eprintln!("[{:?}] {} (satır {}, sütun {})", self.kind, self.message, self.line, self.col);
        if let Some(ref token) = self.token {
            eprintln!("Token: {}", token);
        }
        if let Some(ref snip) = self.snippet {
            eprintln!("--> {}", snip);
        }
    }
} 