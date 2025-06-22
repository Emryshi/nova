// src/vm.rs
// NovaLang için kapsamlı yorumlayıcı (interpreter) ve VM

use crate::ast::{Expr, Stmt, Program};
use crate::runtime::Runtime;
use crate::error::{NovaError, NovaErrorKind};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Func {
        params: Vec<String>,
        body: Vec<Stmt>,
        env: Env,
    },
    List(Vec<Value>),
    Map(std::collections::HashMap<String, Value>),
}

pub type Env = HashMap<String, Value>;

pub struct VM {
    pub globals: Env,
    pub runtime: Runtime,
}

impl VM {
    pub fn new() -> Self {
        VM {
            globals: HashMap::new(),
            runtime: Runtime::new(),
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), NovaError> {
        for stmt in &program.body {
            self.eval_stmt(stmt, &mut self.globals.clone())?;
        }
        Ok(())
    }

    fn eval_stmt(&mut self, stmt: &Stmt, env: &mut Env) -> Result<Option<Value>, NovaError> {
        match stmt {
            Stmt::TryCatch { try_block, err_name, catch_block, line, col } => {
                let mut local_env = env.clone();
                match self.eval_block(try_block, &mut local_env) {
                    Ok(_) => Ok(None),
                    Err(e) => {
                        local_env.insert(err_name.clone(), Value::String(e.message));
                        self.eval_block(catch_block, &mut local_env)?;
                        Ok(None)
                    }
                }
            }
            Stmt::Import { name, line, col } => {
                // Basit modül yükleyici: 'moduladi.nova' dosyasını yükle ve çalıştır
                let filename = format!("{}.nova", name);
                match std::fs::read_to_string(&filename) {
                    Ok(code) => {
                        // Lex, parse, semantic, run
                        let mut lexer = crate::lexer::Lexer::new(&code);
                        let mut tokens = Vec::new();
                        loop {
                            let tok = lexer.next_token();
                            if let crate::token::TokenKind::Eof = tok.kind { break; }
                            tokens.push(tok);
                        }
                        let mut parser = crate::parser::Parser::new(tokens);
                        let program = parser.parse_program()?;
                        let mut analyzer = crate::semantic::SemanticAnalyzer::new();
                        analyzer.analyze(&program)?;
                        self.run(&program)?;
                        Ok(None)
                    }
                    Err(_) => Err(crate::error::NovaError::new(crate::error::NovaErrorKind::RuntimeError, "Modül dosyası bulunamadı", *line, *col, None)),
                }
            }
            _ => Ok(None),
        }
    }

    fn eval_expr(&mut self, expr: &Expr, env: &mut Env) -> Result<Value, NovaError> {
        match expr {
            Expr::List(items, line, col) => {
                let mut vals = Vec::new();
                for item in items {
                    vals.push(self.eval_expr(item, env)?);
                }
                Ok(Value::List(vals))
            }
            Expr::Map(pairs, line, col) => {
                let mut map = std::collections::HashMap::new();
                for (k, v) in pairs {
                    let key_val = self.eval_expr(k, env)?;
                    let val_val = self.eval_expr(v, env)?;
                    if let Value::String(s) = key_val {
                        map.insert(s, val_val);
                    } else {
                        return Err(NovaError::new(NovaErrorKind::RuntimeError, "Map anahtarı string olmalı", *line, *col, None));
                    }
                }
                Ok(Value::Map(map))
            }
            Expr::Index { collection, index, line, col } => {
                let coll_val = self.eval_expr(collection, env)?;
                let idx_val = self.eval_expr(index, env)?;
                match (coll_val, idx_val) {
                    (Value::List(list), Value::Int(i)) => {
                        let idx = i as usize;
                        if idx < list.len() {
                            Ok(list[idx].clone())
                        } else {
                            Err(NovaError::new(NovaErrorKind::RuntimeError, "Liste indeksi geçersiz", *line, *col, None))
                        }
                    }
                    (Value::Map(map), Value::String(s)) => {
                        if let Some(val) = map.get(&s) {
                            Ok(val.clone())
                        } else {
                            Err(NovaError::new(NovaErrorKind::RuntimeError, "Map anahtarı bulunamadı", *line, *col, None))
                        }
                    }
                    _ => Err(NovaError::new(NovaErrorKind::RuntimeError, "Geçersiz index işlemi", *line, *col, None)),
                }
            }
            Expr::Lambda { params, body, line, col } => {
                Ok(Value::Func {
                    params: params.clone(),
                    body: vec![Stmt::Expr(*body.clone())],
                    env: env.clone(),
                })
            }
            Expr::Call { func, args, line, col } => {
                let func_val = self.eval_expr(func, env)?;
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.eval_expr(arg, env)?);
                }
                match func_val {
                    Value::Func { params, body, mut env: closure_env } => {
                        if params.len() != arg_vals.len() {
                            return Err(NovaError::new(NovaErrorKind::RuntimeError, "Fonksiyon parametre sayısı uyuşmuyor", *line, *col, None));
                        }
                        for (i, param) in params.iter().enumerate() {
                            closure_env.insert(param.clone(), arg_vals[i].clone());
                        }
                        let mut result = Value::Null;
                        for stmt in &body {
                            if let Some(val) = self.eval_stmt(stmt, &mut closure_env)? {
                                result = val;
                                break;
                            }
                        }
                        Ok(result)
                    }
                    _ => Err(NovaError::new(NovaErrorKind::RuntimeError, "Fonksiyon çağrısı geçersiz", *line, *col, None)),
                }
            }
            _ => Ok(Value::Null),
        }
    }

    fn eval_block(&mut self, block: &Vec<Stmt>, env: &mut Env) -> Result<Option<Value>, NovaError> {
        for stmt in block {
            self.eval_stmt(stmt, env)?;
        }
        Ok(None)
    }
} 