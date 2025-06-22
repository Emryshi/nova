// src/ast.rs
// NovaLang için kapsamlı ve genişletilebilir AST düğümleri

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
        line: usize,
        col: usize,
    },
    Expr(Expr),
    Func {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        line: usize,
        col: usize,
    },
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        line: usize,
        col: usize,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
        line: usize,
        col: usize,
    },
    Return(Expr, usize, usize),
    Block(Vec<Stmt>),
    TryCatch {
        try_block: Vec<Stmt>,
        err_name: String,
        catch_block: Vec<Stmt>,
        line: usize,
        col: usize,
    },
    TryCatchFinally {
        try_block: Vec<Stmt>,
        err_name: String,
        catch_block: Vec<Stmt>,
        finally_block: Vec<Stmt>,
        line: usize,
        col: usize,
    },
    Import {
        name: String,
        line: usize,
        col: usize,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
        line: usize,
        col: usize,
    },
    Class {
        name: String,
        base: Option<String>,
        methods: Vec<Stmt>,
        line: usize,
        col: usize,
    },
    Export {
        name: String,
        value: Expr,
        line: usize,
        col: usize,
    },
    AsyncFunc {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        line: usize,
        col: usize,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64, usize, usize),
    Float(f64, usize, usize),
    String(String, usize, usize),
    Bool(bool, usize, usize),
    Null(usize, usize),
    Identifier(String, usize, usize),
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
        line: usize,
        col: usize,
    },
    UnaryOp {
        op: String,
        expr: Box<Expr>,
        line: usize,
        col: usize,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
        line: usize,
        col: usize,
    },
    Assign {
        name: String,
        value: Box<Expr>,
        line: usize,
        col: usize,
    },
    List(Vec<Expr>, usize, usize),
    Map(Vec<(Expr, Expr)>, usize, usize),
    Index {
        collection: Box<Expr>,
        index: Box<Expr>,
        line: usize,
        col: usize,
    },
    Object {
        class_name: String,
        fields: std::collections::HashMap<String, crate::vm::Value>,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
        line: usize,
        col: usize,
    },
    New {
        class_name: String,
        args: Vec<Expr>,
        line: usize,
        col: usize,
    },
    Await {
        expr: Box<Expr>,
        line: usize,
        col: usize,
    },
} 